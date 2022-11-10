//! Various utility functions for getting and further processing of symbols, tickers, 
//! websocket updates and klines obtained from Binance
#![allow(dead_code)]

use http_req::request;
use serde::{Deserialize};
use std::ops::Deref;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use dec::Decimal64;
use inlinable_string::{InlineString};

/// Parse a String into a `Decimal64`, chop off superfluous zeros
// todo: Make this return Result
pub fn parse_dec(s: &String) -> Decimal64 {
    if let Some(_) = s.find(".") {
        s.trim_end_matches("0").parse().expect("parse_dec: Couldn't parse!")
    } else {
        s.parse().expect("parse_dec: Couldn't parse!")
    }
}

/// Nicely format a `Decimal64`
// todo: move this into `Nice`
pub fn fmt_dec(d: Decimal64) -> String {
    if d.is_infinite() {
        String::from("-")
    } else if d.is_nan() || d.is_signaling_nan() {
        String::from("-")
    } else {
        let mut u = d.coefficient();
        let mut e = d.exponent();
        let mut di = d.digits() as i32;
        while (u/10)*10 == u && u!=0 {
            u /= 10;
            e += 1;
            di -= 1;
        }
        if e+di < 0 { // slash notation
            format!("{}\\{}", -di-e+1, u)
        } else {
            format!("{}", d)
        }
    }
}

/// String type for symbol
pub type Symbol = InlineString;

/// `Info` contains symbol, base, quote and precision
#[derive(Debug, Clone)]
pub struct Info {
    pub symbol: Symbol,
    pub base: Symbol,
    pub quote: Symbol,
    pub volume: Decimal64,
}

impl Info {
    pub fn short_symbol(self: &Self) -> &InlineString {
        if self.quote == "USDT" { &self.base }
        else                { &self.symbol }
    }
}

/// Subset of data returned by api/v3/exchangeInfo, for deserialisation only
#[derive(Debug, Clone, Deserialize)]
struct MarketInfo {
    symbols: Vec<MarketInfoSymbol>
}

/// Subset of data returned by api/v3/exchangeInfo, for deserialisation only
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MarketInfoSymbol {
    symbol: String,
    status: String,
    base_asset: String,
    quote_asset: String

}

/// Get all traded binance symbols (unsorted)
fn _get_infos() -> Result<HashMap<Symbol, Info>, Box<dyn std::error::Error>> {
    let mut writer = Vec::with_capacity(3000000);   // exchangeInfo size is <2MB usually
    if !request::get("https://api.binance.com/api/v3/exchangeInfo", &mut writer)?.status_code().is_success() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Req api/v3/exchangeInfo failed")));
    }
    let cow = String::from_utf8_lossy(&writer);
    let market_info: MarketInfo = serde_json::from_str(cow.deref())?;
    let mut out = HashMap::<Symbol, Info>::new();
    for sym in market_info.symbols.iter() {
        if sym.status == "TRADING" {
            let symbol = InlineString::from(sym.symbol.as_str());
            let base = InlineString::from(sym.base_asset.as_str());
            let quote = InlineString::from(sym.quote_asset.as_str());
            out.insert(symbol.clone(),  Info { symbol: symbol, base: base, quote: quote, volume: Decimal64::NAN});
        }    
    } 
    Ok(out)
}

/// Market information subset as retrieved by API GET /api/v3/ticker/24hr
#[derive(Debug)]
pub struct Market {
    pub price: Decimal64,
    pub volume: Decimal64,
    pub price_change: Decimal64,
}

/// Subset of data returned by api/v3/ticker/24hr, for deserialisation only
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Ticker {
    symbol: String,
    price_change: String,
    quote_volume: String,
    last_price: String
}

/// Get all traded binance symbols
pub fn get_markets<'a>() -> Result<HashMap<Symbol, Market>, Box<dyn std::error::Error>> {
    let mut writer = Vec::with_capacity(1500000);   // 24hr size is <1MB usually
    if !request::get("https://api.binance.com/api/v3/ticker/24hr", &mut writer)?.status_code().is_success() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Req api/v3/ticker/24hr failed")));
    }
    let cow = String::from_utf8_lossy(&writer);
    let tickers: Vec<Ticker> = serde_json::from_str(cow.deref())?;
    let mut out = HashMap::<Symbol, Market>::new();
    for ticker in tickers.iter() {
        let symbol = InlineString::from(ticker.symbol.as_str());
        let price_change: Decimal64 = ticker.price_change.parse()?;
        let vol: Decimal64 = ticker.quote_volume.parse()?;
        let px: Decimal64 = ticker.last_price.parse()?;
        if vol.is_positive() {
            let mkt = Market { price: px, volume: vol, price_change: price_change };
            out.insert(symbol, mkt);
        }
    }
    Ok(out)
}

/// Get all traded binance symbols sorted by trading volume (in USDT)
pub async fn get_infos() -> Result<Vec<Info>, String> {
    let infos = _get_infos().map_err(|e| format!("Get infos failed: {:?}", e))?;
    let markets = get_markets().map_err(|e| format!("Get markets failed: {:?}", e))?;
    let mut out = Vec::<Info>::new();
    for (symbol, mut info) in infos.into_iter() {
        if let Some(market) = markets.get(&symbol) {
            // if the quote ccy is not USDT we try to convert the volume to USDT 
            if info.quote != "USDT" {
                let mut usdt_sym = info.quote.clone();
                usdt_sym.push_str("USDT").map_err(|e| format!("{:?}", e))?;
                if let Some(mkt2) = markets.get(&usdt_sym) {
                    info.volume = market.volume * mkt2.price;
                    out.push(info);
                }
            } else {
                info.volume = market.volume;
                out.push(info);
            }
        }
    }
    Ok(out)
}

#[tokio::test]
async fn test_get_infos() -> Result<(), Box<dyn std::error::Error>> {
    let infos = get_infos().await?;
    assert!(infos.len()>0);
    Ok
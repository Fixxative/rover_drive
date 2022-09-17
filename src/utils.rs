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
    if !request::get("https://api.binance.com/api/v3/exchangeInfo", &mut writer)?.status_code().is_

mod utils;
mod ui;
use crate::{
    utils::*,
    ui::*
};
use std::{
    io,
    time::Duration
};
use termion::{
    event::Key,
    input::TermRead,
    raw::IntoRawMode
};
use tui::{Terminal, backend::TermionBackend};
use tokio_tungstenite::{connect_async};
use tokio::sync::mpsc::UnboundedSender;
use futures_util::{future, StreamExt};
use url::Url;
use clap::{Command};
use version::version;

/// Duration of `sleep` in `listen_keys` loop
const LISTEN_KEYS_SLEEP_MILLIS: u64 = 100;

/// Binance 24h ticker stream endpoint
const URI_WS_TICKER: &str = "wss://stream.binance.com:9443/ws/!ticker@arr";

/// Listen to terminal input.
/// 
/// This is simply an endless loop that reads the terminal input in `LOOP_SPEED` intervals and sends
/// the appropriate message to `tx`.
async fn listen_keys(tx: UnboundedSender<Msg>) -> Result<(), String> {
    let mut stdin = termion::async_stdin().keys();
    loop {
        if let Some(Ok(key)) = stdin.next() {
            match key {
                Key::Char('q') => {
                    tx.send(Msg::Stop).expect("UI failed");
                    break;
                },
                Key::Ctrl('c') => {
                    tx.send(Msg::Stop).expect("UI failed");
                    break;
                },
                Key::Char('l')  => { tx.send(Msg::PriceList).expect("UI failed"); },
                Key::Char('t')  => { tx.send(Msg::PriceTable).expect("UI failed"); },
                Key::Char('%')  => { tx.send(Msg::TogglePercent).expect("UI failed"); },
                Key::Char('x')  => { tx.send(Msg::ToggleExtended).expect("UI failed"); },
                Key::Char('s')  => { tx.send(Msg::Search).expect("UI failed"); },
                Key::Char('h')  => { tx.send(Msg::Help).expect("UI failed"); },
                Key::Char('a')  => { tx.send(Msg::About).expect("UI failed"); },
                Key::Char('g')  => { tx.send(Msg::Graph(None)).expect("UI failed"); },
                Key::Char('0')  => { tx.send(Msg::Graph(Some(0))).expect("UI failed"); },
                Key::Char('1')  => { tx.send(Msg::Graph(Some(1))).expect("UI failed"); },
                Key::Char('2')  => { tx.send(Msg::Graph(Some(2))).expect("UI failed"); },
                Key::Char('3')  => { tx.send(Msg::Graph(Some(3))).expect("UI failed"); },
                Key::Char('4')  => { tx.send(Msg::Graph(Some(4))).expect("UI failed"); },
                Key::Char('5')  => { tx.send(Msg::Graph(Some(5))).expect("UI failed"); },
                Key::Char('6')  => { tx.send(Msg::Graph(Some(6))).expect("UI failed"); },
                Key::Char('7')  => { tx.send(Msg::Graph(Some(7))).expect("UI failed"); },
                Key::Char('8')  => { tx.send(Msg::Graph(Some(8))).expect("UI failed"); },
                Key::Char('9')  => { tx.send(Msg::Graph(Some(9))).expect("UI failed"); },
                Key::Up         => { tx.send(Msg::ArrowUp).expect("UI failed"); },
                Key::Down       => { tx.send(Msg::ArrowDown).expect("UI failed"); },
                Key::Left       => { tx.send(Msg::ArrowLeft).expect("UI failed"); },
                Key::Right      => { tx.send(Msg::ArrowRight).expect("UI failed"); },
                Key::Home       => { tx.send(Msg::Home).expect("UI failed"); },
                Key::Char('\n') => { tx.send(Msg::Enter).expect("UI failed"); },
                Key::Esc        => { tx.send(Msg::Esc).expect("UI failed"); },
                key => { 
                    tx.send(Msg::Msg(format!("Unknown command {:?}", key)))
                      .map_err(|e| format!("UI failed: {:?}", e))?; 
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(LISTEN_KEYS_SLEEP_MILLIS)).await;
    }
    Ok(())
}

/// Websocket stream
async fn ws(uri: &str, ui_tx: UnboundedSender<Msg>) -> Result<(), String> {
    let uri: Url = Url::parse(uri).map_err(|e| format!("Bad url: {:?}", e))?;
    let (ws_stream, response) = match connect_async(uri).await {
        Ok((ws_stream, response)) => { (ws_stream, response) },
        Err(e) => { 
            ui_tx.send(Msg::Msg(format!("Error connecting: {:?}", e)))
                 .map_err(|e| format!("UI failed: {:?}", e))?;
            return Ok(());
        }
    };
    ui_tx.send(Msg::Msg(format!("Websocket connected:\n{:?}", response)))
         .map_err(|e| format!("UI failed: {:?}", e))?;

    let (_, mut read) = ws_stream.split();

    ui_tx.send(Msg::Msg(String::from("Starting..."))).expect("UI failed");
    loop {
        let next = read.next().await;
        let now = now_timestamp();
        match next {
            Some(msg) => {
                match msg {
                    Ok(msg)  => {
                        let msg = msg.to_string();
                        ui_tx.send(Msg::WS(now, msg))
                             .map_err(|e| format!("UI failed: {:?}", e))?;
                    }, 
                    Err(e) => {
                        ui_tx.send(Msg::Msg(format!("Error: {:?}", e)))
                             .map_err(|e| format!("UI failed: {:?}", e))?;
                        return Err(format!("Websocket error: {:?}", e));
                    }
                }
            },
            None => {
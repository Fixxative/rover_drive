
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
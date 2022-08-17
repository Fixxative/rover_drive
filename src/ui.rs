
//! The UI is made of different pages
#![allow(dead_code)]

/// The help page
pub mod help;
/// The about page
pub mod about;
/// The price list page
pub mod price_list;
/// The price table page
pub mod price_table;
/// The graph page
pub mod graph;
/// The search page
pub mod search;
/// Pretty printing of floats and Decimal
pub mod nice;

use crate::utils::*;
use std::cell::RefCell;
use std::rc::Rc;
use tui::{
    backend::Backend,
    style::{Style, Color, Modifier},
    widgets::{Paragraph},
    layout::{Layout, Constraint, Direction, Rect, Alignment},
    text::{Span, Spans},
    terminal::Frame,
};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use std::collections::HashMap;
use chrono::Local;
use std::marker::Copy;
use dec::Decimal64;
use inlinable_string::{InlineString};

/// Stores the relevant market data with some extra rendering information
pub struct MarketState {
    px: Decimal64,
    ts: u64,
    last_px: Decimal64,
    px_24h: Decimal64,
}

impl MarketState {
    /// Create new `MarketState` with NANs.
    fn new() -> Self {
        MarketState { px: Decimal64::NAN, ts: 0, last_px: Decimal64::NAN, px_24h:Decimal64::NAN }
    }
    /// Update `MarketState` with data from `Update`
    fn update(self: &mut Self, update: &Update) {
        self.last_px = self.px;
        self.px = update.px;
        self.px_24h = update.px_24h;
        self.ts = update.ts;
    }
    /// Make a nicely formatted price string
    pub fn price_string(self: &Self) -> String {
        fmt_dec(self.px)
    }
    /// Make a percentage string that has 6 width !TODO! improve
    pub fn percentage_string(self: &Self) -> String {
        let hundred: Decimal64 = "100".parse().expect("INTERNAL ERROR");
        let p = (self.last_px-self.px_24h)/self.px_24h;
        let mut s = if p.is_infinite() || p.is_nan() {
            String::from("-")
        } else {
            format!("{}", p*hundred)
        };
        if p.is_positive() && !(p.is_infinite() || p.is_nan()) { 
            s.insert_str(0,"+");
        } 
        s.truncate(6);
        format!("{:>6}", s)
    } 
    /// Generate a style for this price
    pub fn style(self: &Self) -> Style {
        if self.px > self.last_px {
            Style::default().fg(Color::Green)
        } else if self.px < self.last_px {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        }
    }
    /// Generate a style for this percentage
    pub fn style_percent(self: &Self) -> Style {
        if self.px > self.px_24h {
            Style::default().fg(Color::Green)
        } else if self.px < self.px_24h {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        }
    }
}

/// Messages that the `UI` can receive
#[derive(Debug)]
pub enum Msg {
    WS(u64, String),    // timestamp (millis) and websocket data
    Infos(Vec<Info>),   // Downloaded infos for each symbol
    Msg(String),        // info message to UI
    PriceList,          // On 'l' key press show PriceList
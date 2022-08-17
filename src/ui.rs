
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
    PriceTable,         // On 't' key press show PriceTable
    Graph(Option<u32>), // On 'g' display graph with given time scale, or stored time scale if Nothing
    TogglePercent,      // On '%' key press
    ToggleExtended,     // On 'x' key press
    Search,             // On 's' show the search widget
    ArrowUp,            // On arrow up
    ArrowDown,          // On arrow down
    ArrowLeft,          // On srrow left
    ArrowRight,         // On arrow right
    Home,               // Home Home key reset cursor to top left
    Enter,              // On pressing enter
    Help,               // On 'h' key press show help
    About,              // On 'a' key press show about page
    Esc,                // On ESC go back to previous page
    Stop                // stop ui
}

/// Just tui::Terminal<...>
type Term = tui::Terminal<tui::backend::TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>;

/// All the different pages
#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum UIView {
    PriceList,  // display PriceList
    PriceTable, // display PriceTable
    Graph,      // display graph
    Search,     // display search widget
    Empty,      // display PriceTable
    Help,       // display help
    About,      // display help
}

impl Copy for UIView { }


/// Current state of the `UI`
pub struct UIState {
    message: String,
    markets: HashMap<Symbol, MarketState>,
    latency: u64,
    ui_mode: UIView,
    ui_mode_back: Option<UIView>,       // where to go back to if ESC is pressed
    show_percent: bool,                 // 
    extended: bool,                     // extended view of table page
    ts_last_update: u64,                // ts of last market update
    lookup: Option<HashMap<Symbol, Info>>,
    infos: Option<Vec<Info>>,
    klines: Option<Vec<Bar>>,
    symbol: Symbol,
    time_scale: u32,                    // time scale for graph
    cursor_ix: u16,                     // x position of symbol in search widget
    cursor_iy: u16,                     // y position of symbol in search widget
}

impl UIState {
    /// New `UIState` with empty fields, 0 latency, ui_mode `PriceList`
    fn new() -> Self {
        UIState { 
            message: String::new(), 
            markets: HashMap::new(),
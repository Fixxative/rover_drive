
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
            latency: 0,
            ui_mode: UIView::Empty,
            ui_mode_back: None,
            show_percent: false,
            extended: true,
            ts_last_update: 0,
            lookup: None,
            infos: None,
            klines: None,
            symbol: InlineString::from("BTCUSDT"),
            time_scale: 0,
            cursor_ix: 0,
            cursor_iy: 0,
        }
    }
    fn update(self: &mut Self, updates: &Vec<Update>) {
        if let Some(lookup) = &self.lookup {
            for u in updates {
                if u.ts > self.ts_last_update { self.ts_last_update = u.ts; }
                let info = lookup.get(&u.symbol);
                if let Some(_) = info {
                    self.markets.entry(u.symbol.clone()).or_insert(MarketState::new()).update(&u);
                }
            }
        }
    }
}
/// Encapsulates the `UI`
pub struct UI {
    pub tx: UnboundedSender<Msg>,
    pub handle: tokio::task::JoinHandle<()>,
}

impl UI {
    /// Create new `UI`
    pub fn new(mut terminal: Term) -> Self {
        terminal.clear().expect("Terminal failed!");
        let (tx, mut rx) = unbounded_channel();
        let handle = tokio::spawn( async move {
            let mut state = UIState::new();
            let mut buf: Vec<Update> = Vec::with_capacity(2000);    // buffer for parse_updates
            let mut cursor_moved: bool = false;                     // used for setting message after draw is done
            while let Some(msg) = rx.recv().await {
                match msg {
                    Msg::Infos(infos_) => {
                        state.infos = Some(infos_.iter().cloned().filter(|i| i.quote != "TUSD" && i.quote != "BUSD" && i.quote != "USDC").collect());
                        state.lookup = Some(infos_to_lookup(&infos_));
                        state.ui_mode = UIView::PriceList;
                    },
                    Msg::WS(ts_rec, msg) => {
                        if let Ok(us) = parse_updates(&msg, &mut buf) {
                            state.update(&us);
                        } else if let Ok(ts) = msg.parse::<u64>() {
                            state.latency = ts_rec-ts;
                        } else {
                            state.message = format!("{:?}", msg);
                            break;
                        }
                    },
                    Msg::Msg(msg) => {
                        state.message = msg;
                    },
                    Msg::PriceList => {
                        state.ui_mode = UIView::PriceList;
                        state.message = String::from("Show price list");
                    },
                    Msg::PriceTable => {
                        state.ui_mode = UIView::PriceTable;
                        state.message = String::from("Show price table");
                    },
                    Msg::Graph(scale) => {
                        state.time_scale = scale.unwrap_or(state.time_scale);
                        UI::graph(&mut state, &mut terminal).await;
                    },
                    Msg::Search => {
                        state.ui_mode_back = Some(state.ui_mode);
                        state.ui_mode = UIView::Search;
                        state.message = String::from("Select symbol");
                    },
                    Msg::ArrowUp => {
                        if state.ui_mode == UIView::Search {
                            if state.cursor_iy > 0 { 
                                state.cursor_iy -= 1;
                                cursor_moved = true;
                            }
                        }
                    },
                    Msg::ArrowDown => {
                        if state.ui_mode == UIView::Search {
                            state.cursor_iy += 1;   // ! height needs to be checked elsewhere!
                            cursor_moved = true;
                        }
                    },
                    Msg::ArrowLeft => {
                        if state.ui_mode == UIView::Search {
                            if state.cursor_ix > 0 { 
                                state.cursor_ix -= 1;
                                cursor_moved = true;
                            }
                        }
                    },
                    Msg::ArrowRight => {
                        if state.ui_mode == UIView::Search {
                            state.cursor_ix += 1;   // ! width needs to be checked elsewhere!
                            cursor_moved = true;
                        }
                    },
                    Msg::Home => {
                        if state.ui_mode == UIView::Search {
                            state.cursor_ix = 0;
                            state.cursor_iy = 0;
                            cursor_moved = true;
                        }
                    },
                    Msg::Enter => {
                        if state.ui_mode == UIView::Search {
                            state.message = format!("Graph {}", state.symbol);
                            state.ui_mode_back = Some(state.ui_mode);
                            state.ui_mode = UIView::Graph;
                            UI::graph(&mut state, &mut terminal).await;
                        }
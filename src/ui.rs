
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
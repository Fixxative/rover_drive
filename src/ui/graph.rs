///! Widget `Graph`
use crate::utils::*;
use crate::ui::nice::{f64_nice_range, Nice};
use tui::{
    style::{Style, Color, Modifier},
    widgets::{Axis, Chart, Widget, Block, Dataset, GraphType, Paragraph},
    layout::{Rect},
    text::{Span},
    buffer::{Buffer},
    symbols
};
use chrono::{Utc, prelude::DateTime};
use std::time::{UNIX_EPOCH, Duration};
use inlinable_string::InlineString;


/// Widget Graph
/// 
/// Shows a time/closing-price graph of a symbol.
pub struct Graph<'a> {
    symbol: Symbol,
    infos: &'a Vec<Info>,   // sorted list of `Info`
    klines: &'a Vec<Bar>,
    interval: Interval,     // 1m, 3m, 5m, etc.
}

impl<'a> Graph<'a> {
    pub fn new(infos: &'a Vec<Info>, klines: &'a Vec<Bar>, interval: Interval, symbol: Symbol) -> Graph<'a> {
     
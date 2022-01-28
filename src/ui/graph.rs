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
        Graph { symbol: symbol, infos: infos, klines: klines, interval: interval }
    }
}

impl<'a> Widget for Graph<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.klines.len() < 1 {
            Paragraph::new("No data!")
            .style(Style::default().fg(Color::Red))
            .block(
                Block::default()
                    .style(Style::default().fg(Color::White))
                    .title("Error")
            ).render(area, buf);
            return;
        }
        let mut t_min: f64 = f64::MAX;
        let mut t_max: f64 = 0.0;
        let mu

///! Widget `PriceList`
use crate::utils::*;
use crate::ui::MarketState;
use tui::{
    style::{Style, Color, Modifier},
    widgets::{Widget},
    layout::{Rect},
    text::{Span, Spans},
    buffer::{Buffer}
};
use std::collections::HashMap;

/// Widget PriceList
pub struct PriceList<'a> {
    infos: &'a Vec<Info>,                       // sorted list of `Info`
    markets: &'a HashMap<Symbol, MarketState>,  // map symbol to `MarketState`
    show_percent: bool                          // flag indicating whether % change should be shown
}

impl<'a> PriceList<'a> {
    pub fn new(infos: &'a Vec<Info>, markets: &'a HashMap<Symbol, MarketState>, show_percent: bool) -> PriceList<'a> {
        PriceList {infos: infos, markets: markets, show_percent: show_percent }
    }
    fn render_info(self: &Self, info: &Info, width: usize) -> Spans<'a> {
        let grey = Style::default().fg(Color::Gray);
        let mkt = self.markets.get(&info.symbol);
        let mut symbol = info.short_symbol().clone();
        while symbol.len() < width { symbol.push(' ').unwrap_or(()); } // format! with {:<width$} does not work!
        let symbol_span = Span::styled(format!("{} ",symbol), 
                                        Style::default().add_modifier(Modifier::BOLD)
                                                        .add_modifier(Modifier::ITALIC));
        if self.show_percent {
            let percentage = mkt.map(|s| String::from(" ")+&s.percentage_string()).unwrap_or(String::from("-"));
            let percentage_span = Span::styled(percentage, mkt.map(|m| m.style_percent()).unwrap_or(grey));
            Spans::from(vec![symbol_span, percentage_span])
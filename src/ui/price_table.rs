
///! Widget `PriceList`
use crate::utils::*;
use crate::ui::MarketState;
use tui::{
    style::{Style, Color, Modifier},
    widgets::{Widget},
    layout::Rect,
    text::{Span, Spans},
    buffer::{Buffer}
};
use std::collections::HashMap;

/// Widget PriceList
pub struct PriceTable<'a> {
    infos: &'a Vec<Info>,                       // sorted list of `Info`
    markets: &'a HashMap<Symbol, MarketState>,  // map symbol to `MarketState`
    show_percent: bool,                         // flag indicating whether % change should be shown
    extended: bool,                             // flag indicating extended view vs. reduced
    quotes: Vec<Symbol>,
    bases: Vec<Symbol>,
}
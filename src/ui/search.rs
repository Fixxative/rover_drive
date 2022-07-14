///! Widget `Search`
use crate::utils::*;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use tui::{
    style::{Style, Color, Modifier},
    widgets::{Widget},
    layout::{Rect},
    text::{Span, Spans},
    buffer::{Buffer}
};


/// Widget Search
pub struct Search<'a> {
    symbol_width: usize,    // width of longest symbol in info
    infos: &'a Vec<Info>,
    pub ref_i_symbol: Rc<RefCell<usize>>,    // index of selected symbol in infos (interior mutablity via Rc<RefCell<_>>)
    pu
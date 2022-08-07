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
    pub ref_cursor: Rc<RefCell<(u16, u16)>>,    // cursor position ix and iy (interior mutablity via Rc<RefCell<_>>)
}

impl<'a> Search<'a> {
    pub fn new(infos: &'a Vec<Info>, ref_i_symbol: Rc<RefCell<usize>>, 
               ref_cursor: Rc<RefCell<(u16,u16)>>) -> Search<'a> {
        let width = infos.iter().map(|info| info.symbol.len()).max().unwrap_or(8);
        Search { 
            symbol_width: width,
            infos: infos,
            ref_i_symbol: ref_i_symbol,
            ref_cursor: ref_cursor,
        }
    }
}

impl<'a> Widget for Search<'a> {
    fn render(self: Self, area: Rect, buf: &mut Buffer) {
        let mut x: u16 = 0;   // x position in area
        let mut y: u16 = 0;   // y position in area
        let mut ix: u16 = 0;  // x position in table of symbols
        let mut iy: u16 = 0;  // y position in table of symbols (always same as variable `y`, actually)
        let mut cursor: RefMut<(u16, u16)> = self.ref_cursor.borrow_mut();
        let height = area.height as u16;
        let width = area.width as u16;
        let s
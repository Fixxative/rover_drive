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
pub
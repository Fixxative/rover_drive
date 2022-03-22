/// The help page
use tui::{
    style::{Style, Color, Modifier},
    widgets::{Block, Borders, Paragraph},
    text::{Span, Spans},
};

pub fn help<'a>() -> Paragraph<'a> {
    let help: Vec<(&str, &str)> = vec!
    [ ("h",    "Disp
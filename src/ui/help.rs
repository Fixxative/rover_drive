/// The help page
use tui::{
    style::{Style, Color, Modifier},
    widgets::{Block, Borders, Paragraph},
    text::{Span, Spans},
};

pub fn help<'a>() -> Paragraph<'a> {
    let help: Vec<(&str, &str)> = vec!
    [ ("h",    "Display help")
    , ("l",    "Show price list")
    , ("t",    "Show price table")
    , ("g",    "Show graph at current time scale")
    , ("0..9", "Show graph at time scale 0 to 9 (1m to 1d)")
    , ("s",    "Select symbol")
    , ("Home", "Set cursor to t
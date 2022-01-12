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
p
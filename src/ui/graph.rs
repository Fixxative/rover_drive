///! Widget `Graph`
use crate::utils::*;
use crate::ui::nice::{f64_nice_range, Nice};
use tui::{
    style::{Style, Color, Modifier},
    widgets::{Axis, Chart, Widget, Block, Dataset, GraphType, Paragraph},
    layout::{
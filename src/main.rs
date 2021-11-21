
mod utils;
mod ui;
use crate::{
    utils::*,
    ui::*
};
use std::{
    io,
    time::Duration
};
use termion::{
    event::Key,
    input::TermRead,
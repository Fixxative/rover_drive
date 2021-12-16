/// The about page
use tui::{
    style::{Style, Color, Modifier},
    widgets::{Paragraph},
    layout::{Alignment, Rect, Layout, Direction, Constraint},
    text::{Span, Spans},
    backend::Backend,
    terminal::Frame,
};
use version::version;

/// The about paragraph
fn about<'a>() -> (Paragraph<'a>, u16) {
    let txt = vec![
        Spans::from(Span::styled("     ####    #####    ######  ##   ##  ####      ######  ##   ##  #######  ", Style::default().fg(Color::LightCyan))),
        Spans::from(Span::styled("    ##  ##  ### ###     ##    ###  ##   ##         ##    ##   ##   ##   #  ", Style::default().fg(Color::LightCyan))),
        Spans::from(Span::styled("   ##       ##   ##     ##    #### ##   ##         ##    ##   ##   ##      ", Style::default().fg(Color::LightCyan))),
        Spans::from(Span::styled("   ##       ##   ##     ##    #######   ##         ##     ## ##    ####    ", Style::default().fg(Color::LightCyan))),
        Spans::from(Span::styled("   ##       ##   ##     ##    ## ####   ##         ##     ## ##    ##      ", Style::default().fg(Color::LightCyan))),
        Spans::from(Span::styled("    ##  ##  ### ###     ##    ##  ###   ##  ##     ##      ###     ##   #  ", Style::default().fg(Color::LightCyan))),
        Spans::from(Span::styled("     ####    #####    ######  ##   ##  #######   ######    ###    #######  ", Style::default().fg(Color::LightCyan))),
        Spans::from(V
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
        Spans::from(Vec::new()),
        Spans::from(Vec::new()),
        Spans::from(Span::styled("Live cryptocurrency prices CLI",
                                 Style::default().fg(Color::LightCyan).add_modifier(Modifier::ITALIC))),
        Spans::from(Vec::new()),
        Spans::from(Vec::new()),
        Spans::from(Span::styled("(c) Mayer Analytics, GPL-3.0", Style::default().fg(Color::Red))),
        Spans::from(Vec::new()),
        Spans::from(Span:
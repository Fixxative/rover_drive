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
        Spans::from(Span::styled("https://github.com/mayeranalytics/coinlive", Style::default().fg(Color::Green))),
        Spans::from(Vec::new()),
        Spans::from(Span::styled(format!("Version {}", version!()), Style::default().fg(Color::Gray))),
    ];
    let h = txt.len() as u16;
    let p = Paragraph::new(txt)
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center);
 
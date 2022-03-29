use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

pub fn draw<'a>() -> Paragraph<'a> {
    Paragraph::new(Span::styled(
        "Loading...",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    )
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true })
}

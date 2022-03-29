use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

pub fn draw<'a>() -> Paragraph<'a> {
    Paragraph::new(format!("Picterm v{}", env!("CARGO_PKG_VERSION")))
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(Block::default().style(Style::default().fg(Color::White)))
}

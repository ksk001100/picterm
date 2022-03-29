use crate::app::state::AppState;
use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn draw<'a>(state: &'a AppState) -> Paragraph<'a> {
    let result = if let Some(current_image) = state.get_current_image() {
        current_image
    } else {
        vec![]
    };

    Paragraph::new(result)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
        .alignment(Alignment::Center)
}

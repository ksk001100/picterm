use tui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn draw<'a, 'b>(search_term: &'a str) -> Paragraph<'b>
where
    'a: 'b,
{
    Paragraph::new(format!("{}█", search_term)).block(
        Block::default()
            .title("Search")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    )
}

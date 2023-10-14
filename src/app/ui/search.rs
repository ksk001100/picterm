use tui::widgets::{Block, Borders, Paragraph};

pub fn draw<'a, 'b>(search_term: &'a str) -> Paragraph<'b>
where
    'a: 'b,
{
    let block = Block::default().title("Search").borders(Borders::ALL);
    let paragraph = Paragraph::new(format!("{}█", search_term)).block(block);
    paragraph
}

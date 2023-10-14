use crate::app::state::AppState;
use tui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

pub fn draw<'a>(state: &AppState) -> List<'a> {
    let list_items: Vec<ListItem> = state
        .get_paths()
        .iter()
        .map(|img| {
            ListItem::new(
                img.file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap(),
            )
        })
        .collect();

    List::new(list_items).highlight_symbol(">>").block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    )
}

use crate::app::state::AppState;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use tui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

pub fn draw<'a>(state: &AppState, search_term: &str) -> List<'a> {
    let matcher = SkimMatcherV2::default();

    let list_items: Vec<ListItem> = state
        .get_paths()
        .iter()
        .filter_map(|img| {
            let file_name = img
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();

            match matcher.fuzzy_match(&file_name, search_term) {
                Some(_) => Some(ListItem::new(file_name)),
                None => None,
            }
        })
        .collect();

    List::new(list_items).highlight_symbol(">>").block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    )
}

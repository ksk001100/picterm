use crate::app::Actions;
use tui::{
    layout::Constraint,
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
};

pub fn draw(actions: &Actions) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut rows = vec![];
    for action in actions.actions().iter() {
        let keys: Vec<String> = action.keys().iter().map(|k| k.to_string()).collect();
        let key = keys.join(", ");
        let row = Row::new(vec![
            Cell::from(Span::styled(key, key_style)),
            Cell::from(Span::styled(action.to_string(), help_style)),
        ]);
        rows.push(row);
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .widths(&[Constraint::Length(30), Constraint::Percentage(70)])
        .column_spacing(1)
}

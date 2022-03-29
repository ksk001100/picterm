use crate::app::state::AppState;
use byte_unit::Byte;
use tui::{
    layout::Constraint,
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
};

pub fn draw<'a>(state: &AppState) -> Table<'a> {
    let key_style = Style::default().fg(Color::LightCyan);
    let value_style = Style::default().fg(Color::Gray);

    let rows = if let Some(image_info) = state.get_current_image_info() {
        let size = Byte::from(image_info.size)
            .get_appropriate_unit(false)
            .to_string();

        vec![
            Row::new(vec![
                Cell::from(Span::styled("Name", key_style)),
                Cell::from(Span::styled(image_info.name, value_style)),
            ]),
            Row::new(vec![
                Cell::from(Span::styled("Dimensions", key_style)),
                Cell::from(Span::styled(
                    format!("{}x{}", image_info.dimensions.0, image_info.dimensions.1),
                    value_style,
                )),
            ]),
            Row::new(vec![
                Cell::from(Span::styled("Size", key_style)),
                Cell::from(Span::styled(size, value_style)),
            ]),
        ]
    } else {
        vec![]
    };

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .widths(&[Constraint::Length(15), Constraint::Percentage(85)])
        .column_spacing(1)
}

use crate::app::{state::AppState, Actions, App};
use byte_unit::Byte;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Wrap,
    },
    Frame,
};

pub fn draw<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let size = rect.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Percentage(90)])
        .margin(1)
        .split(size);

    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[0]);

    let title = draw_title();
    rect.render_widget(title, header_chunks[0]);

    let info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(header_chunks[1]);

    let help = draw_help(app.actions());
    rect.render_widget(help, info_chunks[0]);

    let info = draw_info(app.state());
    rect.render_widget(info, info_chunks[1]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[1]);

    app.state
        .set_term_size(body_chunks[1].width as u32, body_chunks[1].height as u32);

    let mut state = ListState::default();
    state.select(app.state.get_index());
    let image_list = draw_image_list(app.state());
    rect.render_stateful_widget(image_list, body_chunks[0], &mut state);

    if app.is_loading() && app.state.get_current_image().is_some() {
        let loading = draw_loading();
        rect.render_widget(loading, body_chunks[1]);
    } else {
        let image = draw_image(app.state());
        rect.render_widget(image, body_chunks[1]);
    }
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new(format!("Picterm v{}", env!("CARGO_PKG_VERSION")))
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(Block::default().style(Style::default().fg(Color::White)))
}

fn draw_info<'a>(state: &AppState) -> Table<'a> {
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

fn draw_help(actions: &Actions) -> Table {
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

fn draw_image_list<'a>(state: &AppState) -> List<'a> {
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

fn draw_image<'a>(state: &'a AppState) -> Paragraph<'a> {
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

fn draw_loading<'a>() -> Paragraph<'a> {
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

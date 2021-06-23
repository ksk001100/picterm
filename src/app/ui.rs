use crate::app::state::AppState;
use crate::app::Actions;
use crate::app::App;
use crate::image::image_fit_size;
use image::Rgba;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{
    Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table,
};
use tui::Frame;

pub fn draw<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let size = rect.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Percentage(90)])
        .margin(1)
        .split(size);

    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[0]);

    let title = draw_title();
    rect.render_widget(title, header_chunks[0]);

    let help = draw_help(app.actions());
    rect.render_widget(help, header_chunks[1]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[1]);

    app.state.set_term_size(body_chunks[1].width.clone() as u32, body_chunks[1].height.clone() as u32);

    let mut state = ListState::default();
    state.select(app.state.get_index());
    let image_list = draw_image_list(app.state());
    rect.render_stateful_widget(image_list, body_chunks[0], &mut state);

    let image = draw_image(app.state());
    rect.render_widget(image, body_chunks[1]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Picterm")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(Block::default().style(Style::default().fg(Color::White)))
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
        .widths(&[Constraint::Length(20), Constraint::Percentage(80)])
        .column_spacing(1)
}

fn draw_image_list<'a>(state: &AppState) -> List<'a> {
    let list_items: Vec<ListItem> = state
        .get_images()
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

fn draw_image<'a>(state: &AppState) -> Paragraph<'a> {
    let mut result = vec![];

    if let Some(img) = state.get_image() {
        if let Some(size) = state.get_fit_size() {
            let (w, h) = size;

            let imgbuf = img
                .resize_exact(w, h, image::imageops::FilterType::Triangle)
                .to_rgba8();
            let (width, height) = imgbuf.dimensions();

            for y in 0..height {
                let mut line = vec![];
                for x in 0..width {
                    let pixel = imgbuf.get_pixel(x, y);
                    let Rgba(data) = *pixel;

                    if data[3] == 0 {
                        line.push(Span::styled(" ", Style::default().bg(Color::White)));
                    } else {
                        line.push(Span::styled(
                            " ",
                            Style::default().bg(Color::Rgb(data[0], data[1], data[2])),
                        ));
                    }
                }
                result.push(Spans::from(line));
            }
        }
    }

    Paragraph::new(result)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
        .alignment(Alignment::Center)
}

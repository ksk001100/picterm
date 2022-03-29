mod help;
mod image;
mod image_list;
mod info;
mod loading;
mod title;

use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::ListState,
    Frame,
};

pub fn draw<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let size = rect.size();

    let main_chunks = main_layout(size);
    let header_chunks = header_layout(main_chunks[0]);
    let body_chunks = body_layout(main_chunks[1]);
    let info_chunks = info_layout(header_chunks[1]);

    let title = title::draw();
    let help = help::draw(app.actions());
    let info = info::draw(app.state());
    let image_list = image_list::draw(app.state());

    rect.render_widget(title, header_chunks[0]);
    rect.render_widget(help, info_chunks[0]);
    rect.render_widget(info, info_chunks[1]);

    let mut state = ListState::default();
    state.select(app.state.get_index());
    rect.render_stateful_widget(image_list, body_chunks[0], &mut state);

    let w = body_chunks[1].width as u32;
    let h = body_chunks[1].height as u32;
    app.state.set_term_size(w, h);

    if app.is_loading() && app.state.get_current_image().is_some() {
        let loading = loading::draw();
        rect.render_widget(loading, body_chunks[1]);
    } else {
        let image = image::draw(app.state());
        rect.render_widget(image, body_chunks[1]);
    }
}

fn main_layout(rect: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Percentage(90)])
        .margin(1)
        .split(rect)
}

fn header_layout(rect: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(rect)
}

fn info_layout(rect: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(rect)
}

fn body_layout(rect: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(rect)
}

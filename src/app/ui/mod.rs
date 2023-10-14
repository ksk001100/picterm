mod help;
mod image;
mod image_list;
mod info;
mod loading;
mod search;
mod title;

use crate::app::App;
use std::rc::Rc;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::ListState,
    Frame,
};

use super::state::AppMode;

pub fn draw<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let size = rect.size();

    let main_chunks = main_layout(size);
    let header_chunks = header_layout(main_chunks[0]);
    let body_chunks = body_layout(main_chunks[1]);
    let info_chunks = info_layout(header_chunks[1]);
    let search_chunks = search_layout(body_chunks[0]);
    let mut body_chunk = body_chunks[0];

    let title = title::draw();
    let help = help::draw(app.actions());
    let info = info::draw(app.state());

    rect.render_widget(title, header_chunks[0]);
    rect.render_widget(help, info_chunks[0]);
    rect.render_widget(info, info_chunks[1]);

    let search_term = app.state.get_search_term();
    let image_list = image_list::draw(app.state(), search_term);

    if app.state.get_app_mode() == AppMode::Search {
        let block = search::draw(app.state.get_search_term());
        rect.render_widget(block, search_chunks[1]);
        body_chunk = search_chunks[0];
    }

    let mut state = ListState::default();
    state.select(app.state.get_index());
    rect.render_stateful_widget(image_list, body_chunk, &mut state);

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

fn main_layout(rect: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(9), Constraint::Percentage(90)])
        .margin(1)
        .split(rect)
}

fn header_layout(rect: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(rect)
}

fn info_layout(rect: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(rect)
}

fn body_layout(rect: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(rect)
}

fn search_layout(rect: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Length(1)].as_ref())
        .split(rect)
}

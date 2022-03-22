pub mod actions;
pub mod state;
pub mod ui;

use self::actions::Actions;
use self::state::AppState;
use crate::app::actions::Action;
use crate::image::image_fit_size;
use crate::inputs::key::Key;
use crate::io::IoEvent;
use image::Rgba;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

#[derive(Clone)]
pub struct App<'a> {
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    actions: Actions,
    is_loading: bool,
    state: AppState<'a>,
}

impl<'a> App<'a> {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>) -> Self {
        let actions = vec![Action::Quit].into();
        let is_loading = false;
        let mut state = AppState::default();

        Self {
            io_tx,
            actions,
            is_loading,
            state,
        }
    }

    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            match action {
                Action::Quit => AppReturn::Exit,
                Action::Increment => {
                    self.state.increment_index();
                    if !self.is_loading {
                        self.load_image().await;
                    }
                    AppReturn::Continue
                }
                Action::Decrement => {
                    self.state.decrement_index();
                    if !self.is_loading {
                        self.load_image().await;
                    }
                    AppReturn::Continue
                }
            }
        } else {
            AppReturn::Continue
        }
    }

    pub async fn update_on_tick(&mut self) -> AppReturn {
        AppReturn::Continue
    }

    pub async fn dispatch(&mut self, action: IoEvent) {
        self.is_loading = true;
        if self.io_tx.send(action).await.is_err() {
            self.is_loading = false;
        };
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn state_mut(&'a mut self) -> &'a mut AppState {
        &mut self.state
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn initialized(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        let path = if args[1..].is_empty() { "./" } else { &args[1] };
        self.actions = vec![Action::Quit, Action::Increment, Action::Decrement].into();
        self.state = AppState::initialized(path);
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }

    async fn load_image(&mut self) {
        let mut result = vec![];

        if let Some(index) = self.state.get_index() {
            if let Some(path) = self.state.get_image(index) {
                if let Some(term_size) = self.state.get_term_size() {
                    if let Ok(img) = image::open(path) {
                        let (w, h) = image_fit_size(&img, term_size.width, term_size.height);

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
                                    line.push(Span::from(" "));
                                } else {
                                    line.push(Span::styled(
                                        " ",
                                        Style::default().bg(Color::Rgb(data[0], data[1], data[2])),
                                    ));
                                }
                            }
                            result.push(Spans::from(line));
                        }

                        self.state.set_current_image(result);
                    }
                }
            }
        }
    }
}

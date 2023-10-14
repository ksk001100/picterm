pub mod actions;
pub mod state;
pub mod ui;

use crate::{
    app::{
        actions::{Action, Actions},
        state::AppState,
    },
    inputs::key::Key,
    io::IoEvent,
    utils::ImageMode,
};

use self::state::AppMode;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub image_mode: ImageMode,
}

#[derive(Clone)]
pub struct App<'a> {
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    actions: Actions,
    is_loading: bool,
    pub state: AppState<'a>,
    pub config: AppConfig,
}

impl<'a> App<'a> {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>, image_mode: ImageMode) -> Self {
        let actions = vec![Action::Quit].into();
        let is_loading = false;
        let state = AppState::default();
        let config = AppConfig { image_mode };

        Self {
            io_tx,
            actions,
            is_loading,
            state,
            config,
        }
    }

    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        match self.state.get_app_mode() {
            AppMode::Normal => {
                if let Some(action) = self.actions.find(key) {
                    match action {
                        Action::Quit => AppReturn::Exit,
                        Action::Increment => {
                            self.state.increment_index();
                            AppReturn::Continue
                        }
                        Action::Decrement => {
                            self.state.decrement_index();
                            AppReturn::Continue
                        }
                        Action::Show => {
                            self.dispatch(IoEvent::LoadImage).await;
                            AppReturn::Continue
                        }
                        Action::Search => {
                            self.state.set_app_mode(AppMode::Search);
                            AppReturn::Continue
                        }
                    }
                } else {
                    AppReturn::Continue
                }
            }
            AppMode::Search => {
                let search_term = self.state.get_search_term();

                match key {
                    Key::Backspace => self
                        .state
                        .set_search_term(search_term[..search_term.len() - 1].to_string()),
                    Key::Esc => {
                        self.state.set_search_term("".to_string());
                        self.state.set_app_mode(AppMode::Normal);
                    }
                    Key::Enter => {
                        self.state.set_app_mode(AppMode::Normal);
                    }
                    oth => self
                        .state
                        .set_search_term(format!("{}{}", search_term, oth.key_char())),
                }

                self.state.filter_paths();
                return AppReturn::Continue;
            }
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

    pub fn initialized(&mut self, path: &str) {
        self.actions = vec![
            Action::Quit,
            Action::Increment,
            Action::Decrement,
            Action::Show,
            Action::Search,
        ]
        .into();
        self.state = AppState::initialized(path);
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }
}

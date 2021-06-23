pub mod actions;
pub mod state;
pub mod ui;

use self::actions::Actions;
use self::state::AppState;
use crate::app::actions::Action;
use crate::inputs::key::Key;
use crate::io::IoEvent;
use eyre::Result;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

pub struct App {
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    actions: Actions,
    is_loading: bool,
    state: AppState,
}

impl App {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>) -> Self {
        let actions = vec![Action::Quit].into();
        let is_loading = false;
        let state = AppState::default();

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
                    self.state.clear_image();
                    self.state.increment_index();

                    if let Some(index) = self.state.get_index() {
                        if let Some(path) = self.state.get_image_path(index) {
                            self.dispatch(IoEvent::Load(path)).await;
                        }
                    }

                    AppReturn::Continue
                }
                Action::Decrement => {
                    self.state.clear_image();
                    self.state.decrement_index();

                    if let Some(index) = self.state.get_index() {
                        if let Some(path) = self.state.get_image_path(index) {
                            self.dispatch(IoEvent::Load(path)).await;
                        }
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

    pub fn state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub async fn initialized(&mut self) -> Result<()> {
        let args: Vec<String> = std::env::args().collect();
        let path = if args[1..].is_empty() { "./" } else { &args[1] };
        self.actions = vec![Action::Quit, Action::Increment, Action::Decrement].into();
        self.state = AppState::initialized(path).await?;

        Ok(())
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }
}

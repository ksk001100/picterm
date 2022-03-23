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
};

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
    pub state: AppState<'a>,
}

impl<'a> App<'a> {
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
        self.actions = vec![
            Action::Quit,
            Action::Increment,
            Action::Decrement,
            Action::Show,
        ]
        .into();
        self.state = AppState::initialized(path);
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }
}

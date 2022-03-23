pub mod app;
pub mod image;
pub mod inputs;
pub mod io;
pub mod utils;

use crate::app::ui;
use app::{App, AppReturn};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use eyre::Result;
use inputs::{events::Events, InputEvent};
use io::IoEvent;
use std::{io::stdout, sync::Arc, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

pub async fn start_ui<'a>(app: &Arc<tokio::sync::Mutex<App<'a>>>) -> Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let tick_rate = Duration::from_millis(100);
    let mut events = Events::new(tick_rate);

    {
        let mut app = app.lock().await;
        app.dispatch(IoEvent::Initialize).await;
    }

    loop {
        let mut app = app.lock().await;

        terminal.draw(|rect| ui::draw(rect, &mut app))?;

        let result = match events.next().await {
            InputEvent::Input(key) => app.do_action(key).await,
            InputEvent::Tick => app.update_on_tick().await,
        };

        if result == AppReturn::Exit {
            events.close();
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

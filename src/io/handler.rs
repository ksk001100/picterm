use super::IoEvent;
use crate::app::App;
use crate::image::image_fit_size;
use eyre::Result;
use image::Rgba;
use std::sync::Arc;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};

pub struct IoAsyncHandler<'a> {
    app: Arc<tokio::sync::Mutex<App<'a>>>,
}

impl<'a> IoAsyncHandler<'a> {
    pub fn new(app: Arc<tokio::sync::Mutex<App<'a>>>) -> Self {
        Self { app }
    }

    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let _ = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::LoadImage => self.do_load_image().await,
        };

        let mut app = self.app.lock().await;
        app.loaded();
    }

    async fn do_initialize(&mut self) -> Result<()> {
        let mut app = self.app.lock().await;
        app.initialized();

        Ok(())
    }

    async fn do_load_image(&mut self) -> Result<()> {
        let mut app = self.app.lock().await;

        Ok(())
    }
}

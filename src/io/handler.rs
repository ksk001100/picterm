use super::IoEvent;
use crate::app::App;
use eyre::Result;
use std::sync::Arc;
use std::time::Duration;

pub struct IoAsyncHandler {
    app: Arc<tokio::sync::Mutex<App>>,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }

    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let _ = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::Sleep(duration) => self.do_sleep(duration).await,
        };

        let mut app = self.app.lock().await;
        app.loaded();
    }

    async fn do_initialize(&mut self) -> Result<()> {
        let mut app = self.app.lock().await;
        tokio::time::sleep(Duration::from_secs(1)).await;
        app.initialized();

        Ok(())
    }

    async fn do_sleep(&mut self, duration: Duration) -> Result<()> {
        tokio::time::sleep(duration).await;
        let _ = self.app.lock().await;

        Ok(())
    }
}

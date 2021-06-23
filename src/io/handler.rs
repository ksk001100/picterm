use super::IoEvent;
use crate::app::App;
use eyre::Result;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use crate::image::image_fit_size;
use image::{GenericImageView, DynamicImage};

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
            IoEvent::Load(path) => self.do_load(path).await,
        };

        let mut app = self.app.lock().await;
        app.loaded();
    }

    async fn do_initialize(&mut self) -> Result<()> {
        {
            let mut app = self.app.lock().await;
            tokio::time::sleep(Duration::from_secs(1)).await;
            app.initialized().await?;

            if let Some(index) = app.state().get_index() {
                if let Some(path) = app.state().get_image_path(index) {
                    if let Ok(bytes) = tokio::fs::read(path).await {
                        app.state_mut().set_image(&bytes);
                    }
                }
            }
        }

        {
            let mut app = self.app.lock().await;
            tokio::task::block_in_place(move || {
                if let Some(img) = app.state().get_image() {
                    if let Some((w, h)) = app.state().get_term_size() {
                        let (w, h) = image_fit_size(img, w, h);
                        app.state_mut().set_fit_size(w, h);
                    }
                }
            });
        }

        Ok(())
    }

    async fn do_load(&mut self, path: PathBuf) -> Result<()> {
        {
            let mut app = self.app.lock().await;

            if let Ok(bytes) = tokio::fs::read(path).await {
                app.state_mut().set_image(&bytes);
            }
        }

        {
            let mut app = self.app.lock().await;
            tokio::task::block_in_place(move || {
                if let Some(img) = app.state().get_image() {
                    if let Some((w, h)) = app.state().get_term_size() {
                        let (w, h) = image_fit_size(img, w, h);
                        app.state_mut().set_fit_size(w, h);
                    }
                }
            });
        }


        Ok(())
    }
}

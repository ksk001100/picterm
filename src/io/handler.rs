use super::IoEvent;
use crate::app::App;
use eyre::Result;
use std::sync::Arc;
use image::Rgba;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use crate::image::image_fit_size;

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
        let mut result = vec![];

        if let Some(index) = app.state().get_index() {
            if let Some(path) = app.state().get_image(index) {
                if let Ok(img) = image::open(path) {
                    let (w, h) = image_fit_size(&img, rect);

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
                }
            }
        }

        app.state_mut().set_current_image(result);

        Ok(())
    }
}

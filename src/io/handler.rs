use crate::{
    app::{state::ImageInfo, App},
    image::image_fit_size,
    io::IoEvent,
};
use eyre::Result;
use image::{GenericImageView, Rgba};
use std::sync::Arc;
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};

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

        if let Some(index) = app.state.get_index() {
            if let Some(path) = app.state.get_path(index) {
                if let Some(term_size) = app.state.get_term_size() {
                    let p = path.clone();
                    let img = tokio::task::block_in_place(move || image::open(p))?;
                    let name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap()
                        .to_string();
                    let size = match path.metadata() {
                        Ok(metadata) => metadata.len(),
                        Err(_) => 0,
                    };
                    let info = ImageInfo {
                        name,
                        size,
                        dimensions: img.dimensions(),
                    };
                    app.state.set_current_image_info(info);

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

                    app.state.set_current_image(result);
                }
            }
        }

        Ok(())
    }
}

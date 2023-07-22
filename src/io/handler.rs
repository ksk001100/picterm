use crate::{
    app::{state::ImageInfo, App},
    image::image_fit_size,
    io::IoEvent,
    utils::ImageMode,
};
use eyre::Result;
use image::{GenericImageView, LumaA, Rgba};
use std::sync::Arc;
use tui::{
    style::{Color, Style},
    text::{Line, Span},
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
            IoEvent::Initialize(path) => self.do_initialize(&path).await,
            IoEvent::LoadImage => self.do_load_image().await,
        };

        let mut app = self.app.lock().await;
        app.loaded();
    }

    async fn do_initialize(&mut self, path: &str) -> Result<()> {
        let mut app = self.app.lock().await;
        app.initialized(path);

        Ok(())
    }

    async fn do_load_image(&mut self) -> Result<()> {
        let result = Arc::new(tokio::sync::Mutex::new(vec![]));

        let opt_index = {
            let app = self.app.lock().await;
            app.state.get_index()
        };

        let opt_path = {
            let app = self.app.lock().await;
            match opt_index {
                Some(index) => app.state.get_path(index),
                None => None,
            }
        };

        let opt_term_size = {
            let app = self.app.lock().await;
            app.state.get_term_size()
        };

        let mode = {
            let app = self.app.lock().await;
            app.config.image_mode.clone()
        };

        {
            if let Some(path) = opt_path {
                if let Some(term_size) = opt_term_size {
                    let img = tokio::task::block_in_place(|| image::open(&path))?;
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

                    let (w, h) = image_fit_size(&img, term_size.width, term_size.height);
                    let imgbuf = img.resize_exact(w, h, image::imageops::FilterType::Triangle);
                    let (width, height) = imgbuf.dimensions();

                    let mut r = result.lock().await;

                    match mode {
                        ImageMode::Rgba => {
                            let imgbuf = imgbuf.to_rgba8();
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
                                            Style::default()
                                                .bg(Color::Rgb(data[0], data[1], data[2])),
                                        ));
                                    }
                                }
                                (*r).push(Line::from(line))
                            }
                        }
                        ImageMode::GrayScale => {
                            let imgbuf = imgbuf.to_luma_alpha8();
                            for y in 0..height {
                                let mut line = vec![];
                                for x in 0..width {
                                    let pixel = imgbuf.get_pixel(x, y);
                                    let LumaA(data) = *pixel;

                                    if data[1] == 0 {
                                        line.push(Span::from(" "));
                                    } else {
                                        line.push(Span::styled(
                                            " ",
                                            Style::default()
                                                .bg(Color::Rgb(data[0], data[0], data[0])),
                                        ));
                                    }
                                }
                                (*r).push(Line::from(line))
                            }
                        }
                    }

                    let mut app = self.app.lock().await;
                    app.state.set_current_image_info(info);
                    app.state.set_current_image((*r).clone());
                }
            }
        }

        Ok(())
    }
}

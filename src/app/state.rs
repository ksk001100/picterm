use crate::utils;
use std::path::PathBuf;
use tui::text::Spans;

#[derive(Debug, Clone)]
pub enum AppState<'a> {
    Init,
    Initialized {
        images: Vec<PathBuf>,
        selected_index: usize,
        term_size: Option<TermSize>,
        current_image: Option<Vec<Spans<'a>>>,
    },
}

#[derive(Debug, Clone)]
pub struct TermSize {
    pub width: u32,
    pub height: u32,
}

impl<'a> AppState<'a> {
    pub fn initialized(path: &str) -> Self {
        let images = utils::get_image_paths(path);
        let selected_index = 0;
        let current_image = None;
        let term_size = None;
        Self::Initialized {
            images,
            selected_index,
            term_size,
            current_image,
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn set_images(&mut self, imgs: Vec<PathBuf>) {
        if let Self::Initialized { images, .. } = self {
            *images = imgs;
        }
    }

    pub fn get_images(&self) -> Vec<PathBuf> {
        if let Self::Initialized { images, .. } = self {
            images.clone()
        } else {
            vec![]
        }
    }

    pub fn get_image(&self, index: usize) -> Option<PathBuf> {
        if let Self::Initialized { images, .. } = self {
            if images.is_empty() {
                None
            } else {
                Some(images[index].clone())
            }
        } else {
            None
        }
    }

    pub fn increment_index(&mut self) {
        if let Self::Initialized {
            selected_index,
            images,
            ..
        } = self
        {
            if *selected_index < images.len() - 1 {
                *selected_index += 1;
            }
        }
    }

    pub fn decrement_index(&mut self) {
        if let Self::Initialized { selected_index, .. } = self {
            if *selected_index > 0 {
                *selected_index -= 1;
            }
        }
    }

    pub fn get_index(&self) -> Option<usize> {
        if let Self::Initialized { selected_index, .. } = self {
            Some(*selected_index)
        } else {
            None
        }
    }

    pub fn set_term_size(&mut self, width: u32, height: u32) {
        if let Self::Initialized { term_size, .. } = self {
            *term_size = Some(TermSize { width, height });
        }
    }

    pub fn get_term_size(&self) -> Option<TermSize> {
        if let Self::Initialized { term_size, .. } = self {
            term_size.clone()
        } else {
            None
        }
    }

    pub fn set_current_image(&mut self, img: Vec<Spans<'a>>) {
        if let Self::Initialized { current_image, .. } = self {
            *current_image = Some(img);
        }
    }

    pub fn get_current_image(&self) -> Option<Vec<Spans<'a>>> {
        if let Self::Initialized { current_image, .. } = self {
            current_image.clone()
        } else {
            None
        }
    }

    pub fn clear_image(&mut self) {
        if let Self::Initialized { current_image, .. } = self {
            *current_image = None;
        }
    }
}

impl<'a> Default for AppState<'a> {
    fn default() -> Self {
        Self::Init
    }
}

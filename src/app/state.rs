use crate::utils;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum AppState {
    Init,
    Initialized {
        images: Vec<PathBuf>,
        selected_index: usize,
    },
}

impl AppState {
    pub fn initialized(path: &str) -> Self {
        let images = utils::get_image_paths(path);
        let selected_index = 0;
        Self::Initialized {
            images,
            selected_index,
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
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}

use crate::utils;
use eyre::Result;
use image::DynamicImage;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum AppState {
    Init,
    Initialized {
        images: Vec<PathBuf>,
        selected_index: usize,
        img: Option<DynamicImage>,
        term_size: Option<(u32, u32)>,
        fit_size: Option<(u32, u32)>,
    },
}

impl AppState {
    pub async fn initialized(path: &str) -> Result<Self> {
        let images = utils::get_image_paths(path).await?;
        let selected_index = 0;
        Ok(Self::Initialized {
            images,
            selected_index,
            img: None,
            term_size: None,
            fit_size: None
        })
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

    pub fn get_image_path(&self, index: usize) -> Option<PathBuf> {
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

    pub fn set_image(&mut self, bytes: &[u8]) {
        if let Self::Initialized { img, .. } = self {
            *img = Some(image::load_from_memory(bytes).unwrap());
        }
    }

    pub fn get_image(&self) -> Option<&DynamicImage> {
        if let Self::Initialized { img, .. } = self {
            img.as_ref()
        } else {
            None
        }
    }

    pub fn get_term_size(&self) -> Option<(u32, u32)> {
        if let Self::Initialized { term_size, .. } = self {
            *term_size
        } else {
            None
        }
    }

    pub fn set_term_size(&mut self, width: u32, height: u32) {
        if let Self::Initialized { term_size, .. } = self {
            *term_size = Some((width, height))
        }
    }

    pub fn get_fit_size(&self) -> Option<(u32, u32)> {
        if let Self::Initialized { fit_size, .. } = self {
            *fit_size
        } else {
            None
        }
    }

    pub fn set_fit_size(&mut self, width: u32, height: u32) {
        if let Self::Initialized { fit_size, .. } = self {
            *fit_size = Some((width, height))
        }
    }

    pub fn clear_image(&mut self) {
        if let Self::Initialized { img, fit_size, .. } = self {
            *img = None;
            *fit_size = None;
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}

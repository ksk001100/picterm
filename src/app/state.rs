use crate::{image::ImageMode, utils};
use std::path::PathBuf;
use tui::text::Spans;

#[derive(Debug, Clone)]
pub enum AppState<'a> {
    Init,
    Initialized {
        paths: Vec<PathBuf>,
        selected_index: usize,
        term_size: Option<TermSize>,
        current_image: Option<Vec<Spans<'a>>>,
        current_image_info: Option<ImageInfo>,
        mode: ImageMode,
    },
}

#[derive(Debug, Clone)]
pub struct TermSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub name: String,
    pub size: u64,
    pub dimensions: (u32, u32),
}

impl<'a> AppState<'a> {
    pub fn initialized(path: &str, mode: ImageMode) -> Self {
        let paths = utils::get_image_paths(path);
        let selected_index = 0;
        let current_image = None;
        let term_size = None;
        let current_image_info = None;
        Self::Initialized {
            paths,
            selected_index,
            term_size,
            current_image,
            current_image_info,
            mode,
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn get_paths(&self) -> Vec<PathBuf> {
        if let Self::Initialized { paths, .. } = self {
            paths.clone()
        } else {
            vec![]
        }
    }

    pub fn get_path(&self, index: usize) -> Option<PathBuf> {
        if let Self::Initialized { paths, .. } = self {
            if paths.is_empty() {
                None
            } else {
                Some(paths[index].clone())
            }
        } else {
            None
        }
    }

    pub fn increment_index(&mut self) {
        if let Self::Initialized {
            selected_index,
            paths: images,
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

    pub fn set_current_image_info(&mut self, image_info: ImageInfo) {
        if let Self::Initialized {
            current_image_info, ..
        } = self
        {
            *current_image_info = Some(image_info);
        }
    }

    pub fn get_current_image_info(&self) -> Option<ImageInfo> {
        if let Self::Initialized {
            current_image_info, ..
        } = self
        {
            current_image_info.clone()
        } else {
            None
        }
    }

    pub fn set_image_mode(&mut self, m: ImageMode) {
        if let Self::Initialized { mode, .. } = self {
            *mode = m;
        }
    }

    pub fn get_image_mode(&self) -> Option<ImageMode> {
        if let Self::Initialized { mode, .. } = self {
            Some(mode.clone())
        } else {
            None
        }
    }
}

impl<'a> Default for AppState<'a> {
    fn default() -> Self {
        Self::Init
    }
}

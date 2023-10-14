use crate::utils;
use std::path::PathBuf;
use tui::text::Line;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Normal,
    Search,
}

#[derive(Debug, Clone)]
pub enum AppState<'a> {
    Init,
    Initialized {
        paths: Vec<PathBuf>,
        selected_index: usize,
        term_size: Option<TermSize>,
        current_image: Option<Vec<Line<'a>>>,
        current_image_info: Option<ImageInfo>,
        search_term: String,
        app_mode: AppMode,
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
    pub fn initialized(path: &str) -> Self {
        let paths = utils::get_image_paths(path);
        let selected_index = 0;
        let current_image = None;
        let term_size = None;
        let current_image_info = None;
        let search_term = "".to_string();
        let app_mode = AppMode::Normal;
        Self::Initialized {
            paths,
            selected_index,
            term_size,
            current_image,
            current_image_info,
            search_term,
            app_mode,
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
            *selected_index += 1;
            if *selected_index >= images.len() {
                *selected_index = 0;
            }
        }
    }

    pub fn decrement_index(&mut self) {
        if let Self::Initialized {
            selected_index,
            paths: images,
            ..
        } = self
        {
            if images.is_empty() {
                return;
            }

            if *selected_index == 0 {
                *selected_index = images.len();
            }
            *selected_index -= 1;
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

    pub fn set_current_image(&mut self, img: Vec<Line<'a>>) {
        if let Self::Initialized { current_image, .. } = self {
            *current_image = Some(img);
        }
    }

    pub fn get_current_image(&self) -> Option<Vec<Line<'a>>> {
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

    pub fn get_search_term(&self) -> &str {
        let Self::Initialized { search_term, .. } = self else {
            return "";
        };
        search_term.as_ref()
    }

    pub fn set_search_term(&mut self, arg: String) {
        if let Self::Initialized { search_term, .. } = self {
            *search_term = arg;
        }
    }

    pub fn set_app_mode(&mut self, mode: AppMode) {
        if let Self::Initialized { app_mode, .. } = self {
            *app_mode = mode;
        }
    }
    pub fn get_app_mode(&self) -> AppMode {
        let Self::Initialized { app_mode, .. } = self else {
            return AppMode::Normal;
        };
        app_mode.clone()
    }
}

impl<'a> Default for AppState<'a> {
    fn default() -> Self {
        Self::Init
    }
}

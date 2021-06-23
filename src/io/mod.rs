pub mod handler;

use std::path::PathBuf;
use image::DynamicImage;

#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize,
    Load(PathBuf),
}

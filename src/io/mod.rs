pub mod handler;

use crate::image::ImageMode;

#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize(String, ImageMode),
    LoadImage,
}

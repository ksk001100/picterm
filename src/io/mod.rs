pub mod handler;

use std::time::Duration;

#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize,
    Sleep(Duration),
}

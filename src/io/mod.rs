pub mod handler;

#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize,
    Increment,
    Decrement,
    LoadImage,
    ClearImage,
}

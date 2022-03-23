pub mod events;
pub mod key;

use crate::inputs::key::Key;

pub enum InputEvent {
    Input(Key),
    Tick,
}

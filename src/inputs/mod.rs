pub mod events;
pub mod key;

use self::key::Key;

pub enum InputEvent {
    Input(Key),
    Tick,
}

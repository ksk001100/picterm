use crate::inputs::key::Key;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::slice::Iter;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    Quit,
    Increment,
    Decrement,
}

impl Action {
    pub fn iterator() -> Iter<'static, Action> {
        static ACTIONS: [Action; 3] = [Action::Quit, Action::Increment, Action::Decrement];
        ACTIONS.iter()
    }

    pub fn keys(&self) -> &[Key] {
        match self {
            Action::Quit => &[Key::Char('q'), Key::Ctrl('c')],
            Action::Increment => &[Key::Char('j'), Key::Down],
            Action::Decrement => &[Key::Char('k'), Key::Up],
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Action::Quit => "Quit",
            Action::Increment => "Next",
            Action::Decrement => "Prev",
        };
        write!(f, "{}", str)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
    pub fn find(&self, key: Key) -> Option<&Action> {
        Action::iterator()
            .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }

    pub fn actions(&self) -> &[Action] {
        self.0.as_slice()
    }
}

impl From<Vec<Action>> for Actions {
    fn from(actions: Vec<Action>) -> Self {
        let mut map: HashMap<Key, Vec<Action>> = HashMap::new();
        for action in actions.iter() {
            for key in action.keys().iter() {
                match map.get_mut(key) {
                    Some(vec) => vec.push(*action),
                    None => {
                        map.insert(*key, vec![*action]);
                    }
                }
            }
        }
        let errors = map
            .iter()
            .filter(|(_, actions)| actions.len() > 1)
            .map(|(key, actions)| {
                let actions = actions
                    .iter()
                    .map(Action::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Conflict key {} with actions {}", key, actions)
            })
            .collect::<Vec<_>>();
        if !errors.is_empty() {
            panic!("{}", errors.join("; "))
        }

        Self(actions)
    }
}

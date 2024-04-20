use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum Status {
    Menu,
    Play,
}

impl Default for Status {
    fn default() -> Self {
        Self::Menu
    }
}

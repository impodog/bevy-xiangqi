use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum Status {
    Menu,
    Play,
}

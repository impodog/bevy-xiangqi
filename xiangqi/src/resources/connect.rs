use super::*;

#[derive(Debug, Clone)]
pub struct Player {
    pub color: PieceColor,
}

#[derive(Debug, Clone, Default, Resource)]
pub struct Connection {
    pub url: String,
    pub player: Option<Player>,
}

impl Connection {
    pub fn is_connected(&self) -> bool {
        self.player.is_some()
    }
}

pub(super) fn init_connection(mut commands: Commands) {
    commands.insert_resource(Connection::default());
}

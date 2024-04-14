use super::*;

#[derive(Debug, Default, Resource)]
pub struct BoardInfo {
    pub board: Board,
}

impl BoardInfo {
    pub fn se(&self) -> String {
        (&self.board).into()
    }

    pub fn de(&mut self, s: &str) {
        if let Ok(board) = s.try_into() {
            self.board = board;
        }
    }
}

pub(super) fn init_board(mut commands: Commands) {
    commands.insert_resource(BoardInfo::default());
}

pub(super) fn start_game(mut r_info: ResMut<BoardInfo>) {
    r_info.board = Board::default();
}

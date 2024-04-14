mod board;
mod connect;
mod control;
mod images;
mod moves;

pub(super) use crate::prelude::*;
pub use board::*;
pub use connect::*;
pub use control::*;
pub use images::*;
pub use moves::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateEvent>()
            .add_event::<TryMoveEvent>()
            .add_event::<DoMoveEvent>();
        app.add_systems(
            Startup,
            (
                init_images,
                init_board,
                init_connection,
                init_moves,
                init_control,
            ),
        );
        app.add_systems(
            Update,
            (listen_update, verify_move, do_move, listen_click).run_if(in_state(Status::Play)),
        );
    }
}

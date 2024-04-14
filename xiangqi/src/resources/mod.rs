mod board;
mod connect;
mod images;
mod moves;

pub(super) use crate::prelude::*;
pub use board::*;
pub use connect::*;
pub use images::*;
pub use moves::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateEvent>().add_systems(
            Startup,
            (init_images, init_board, init_connection, init_moves),
        );
        app.add_systems(Update, (listen_update,));
    }
}

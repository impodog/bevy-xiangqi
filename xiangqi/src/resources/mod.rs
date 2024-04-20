mod board;
mod connect;
mod control;
mod fonts;
mod images;
mod moves;
mod query;

pub(super) use crate::prelude::*;
pub use board::*;
pub use connect::*;
pub use control::*;
pub use fonts::*;
pub use images::*;
pub use moves::*;
pub use query::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateEvent>()
            .add_event::<TryMoveEvent>()
            .add_event::<DoMoveEvent>()
            .add_event::<ConnectEvent>();
        app.register_request_type::<ConnectResponse>()
            .register_request_type::<QueryResponse>();
        app.add_systems(
            Startup,
            (
                init_images,
                init_board,
                init_connection,
                init_moves,
                init_control,
                init_fonts,
            ),
        );
        app.add_systems(
            Update,
            (
                test_disconnect,
                listen_update,
                verify_move,
                do_move,
                listen_click,
                query_moves,
                respond_moves,
                listen_end_game,
            )
                .run_if(in_state(Status::Play)),
        );
        app.add_systems(Update, (listen_connect_event, update_connection_player));
    }
}

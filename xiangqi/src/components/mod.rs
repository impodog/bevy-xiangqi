mod camera;
mod marker;
mod pieces;
mod win_lose;

pub(super) use crate::prelude::*;
pub use camera::*;
pub use marker::*;
pub use pieces::*;
pub use win_lose::*;

lazy_static::lazy_static! {
    pub static ref PIECE_EACH: f32 = (WIDTH / FILES as f32).min(HEIGHT / RANKS as f32);
}

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        lazy_static::initialize(&PIECE_EACH);
        app.add_systems(Startup, (spawn_camera,));
        app.add_systems(
            OnEnter(Status::Play),
            (start_game, spawn_pieces, spawn_marker).chain(),
        );
        app.add_systems(
            OnExit(Status::Play),
            (end_game, despawn_marker, despawn_win_lose),
        );
        app.add_systems(
            Update,
            (update_pieces, move_marker, listen_win_lose).run_if(in_state(Status::Play)),
        );
    }
}

mod background;
mod camera;
mod pieces;

pub(super) use crate::prelude::*;
pub use background::*;
pub use camera::*;
pub use pieces::*;

lazy_static::lazy_static! {
    pub static ref PIECE_EACH: f32 = (WIDTH / FILES as f32).min(HEIGHT / RANKS as f32);
}

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        lazy_static::initialize(&PIECE_EACH);
        app.add_systems(Startup, (spawn_camera,));
        app.add_systems(OnEnter(Status::Play), (start_game, spawn_pieces).chain());
        app.add_systems(OnExit(Status::Play), (end_game,));
        app.add_systems(Update, (update_pieces,));
    }
}

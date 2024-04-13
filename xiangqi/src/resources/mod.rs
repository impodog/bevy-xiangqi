mod board;
mod images;

pub(super) use crate::prelude::*;
pub use board::*;
pub use images::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_images);
    }
}

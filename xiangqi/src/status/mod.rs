mod states;

pub(super) use crate::prelude::*;
pub use states::*;

pub struct StatusPlugin;

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(Status::Play);
    }
}

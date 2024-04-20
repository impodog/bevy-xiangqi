mod content;

pub use content::*;

pub(super) use crate::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaunchEvent>();
        app.add_systems(Startup, (init_contents,));
        app.add_systems(
            Update,
            (menu_ui, launch_game, boot_game).run_if(in_state(Status::Menu)),
        );
    }
}

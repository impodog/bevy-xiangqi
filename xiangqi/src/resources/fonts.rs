use super::*;

#[derive(Resource)]
pub struct DefaultFont(pub Handle<Font>);

pub(super) fn init_fonts(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(DefaultFont(server.load("CC.ttf")));
}

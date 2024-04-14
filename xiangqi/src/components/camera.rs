use super::*;

#[derive(Component)]
pub struct CameraMarker;

pub(super) fn spawn_camera(mut commands: Commands) {
    commands.spawn((CameraMarker, Camera2dBundle::default()));
}

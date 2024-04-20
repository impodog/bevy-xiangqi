use super::*;

#[derive(Debug, Default, Resource)]
pub struct PrevClick(pub Option<Position>);

pub(super) fn init_control(mut commands: Commands) {
    commands.init_resource::<PrevClick>();
}

pub(super) fn listen_click(
    click: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<CameraMarker>>,
    mut prev: ResMut<PrevClick>,
    mut try_move: EventWriter<TryMoveEvent>,
    connect: Res<Connection>,
) {
    if let Some(ref player) = connect.player {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            if click.just_pressed(MouseButton::Right) {
                prev.0 = None;
            } else if click.just_pressed(MouseButton::Left) {
                if let Some(from) = prev.0 {
                    try_move.send(TryMoveEvent {
                        from,
                        to: locate_position(world_position, player.color),
                    });
                    prev.0 = None;
                } else {
                    prev.0 = Some(locate_position(world_position, player.color));
                }
            }
        }
    }
}

pub(super) fn listen_end_game(key: Res<ButtonInput<KeyCode>>, mut connect: ResMut<Connection>) {
    if key.just_pressed(KeyCode::Escape) {
        connect.player = None;
    }
}

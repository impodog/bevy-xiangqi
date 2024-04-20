use super::*;

#[derive(Component)]
pub struct MarkerMarker;

pub(super) fn spawn_marker(mut commands: Commands, image: Res<MarkerImage>) {
    commands.spawn((
        MarkerMarker,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(*PIECE_EACH, *PIECE_EACH)),
                ..Default::default()
            },
            texture: image.0.clone(),
            ..Default::default()
        },
    ));
}

pub(super) fn despawn_marker(mut commands: Commands, marker: Query<Entity, With<MarkerMarker>>) {
    marker
        .iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive());
}

pub(super) fn move_marker(
    mut marker: Query<&mut Transform, With<MarkerMarker>>,
    prev: Res<PrevClick>,
    connect: Res<Connection>,
) {
    if let Some(ref player) = connect.player {
        marker.iter_mut().for_each(|mut transform| {
            if prev.is_changed() {
                match prev.0 {
                    Some(pos) => {
                        transform.translation = locate_piece(pos, player.color);
                    }
                    None => {
                        transform.translation = Vec3::new(WIDTH * 2.0, HEIGHT * 2.0, 0.0);
                    }
                }
                transform.translation.z = 2.0;
            }
        });
    }
}

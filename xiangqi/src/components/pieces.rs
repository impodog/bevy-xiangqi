use super::*;

#[derive(Component)]
pub struct PieceMarker;

#[derive(Component)]
pub struct TileMarker;

pub fn locate_piece(pos: Position, color: PieceColor) -> Vec3 {
    let result = Vec3::new(
        (pos.file() as f32 - FILES as f32 / 2.0 + 0.5) * (*PIECE_EACH),
        (pos.rank() as f32 - RANKS as f32 / 2.0 + 0.5) * (*PIECE_EACH),
        0.0,
    );
    if color == PieceColor::Black {
        -result
    } else {
        result
    }
}

pub fn locate_position(pos: Vec2, color: PieceColor) -> Position {
    let color = if color == PieceColor::Black {
        -1.0
    } else {
        1.0
    };
    Position::new_int(
        ((pos.y / *PIECE_EACH * color + RANKS as f32 / 2.0) as isize).min(RANKS as isize - 1),
        ((pos.x / *PIECE_EACH * color + FILES as f32 / 2.0) as isize).min(FILES as isize - 1),
    )
}

pub(super) fn start_game(mut info: ResMut<BoardInfo>, mut update: EventWriter<UpdateEvent>) {
    info.board = Board::default();
    update.send(UpdateEvent);
    info!("Game started");
}

pub(super) fn end_game(
    mut commands: Commands,
    mut connect: ResMut<Connection>,
    q_pieces: Query<Entity, Or<(With<PieceMarker>, With<TileMarker>)>>,
) {
    connect.player = None;
    q_pieces.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
    info!("Game ended");
}

pub(super) fn spawn_pieces(
    mut commands: Commands,
    red: Res<RedImages>,
    black: Res<BlackImages>,
    tile: Res<TileImage>,
    board: Res<BoardInfo>,
    connect: Res<Connection>,
) {
    if let Some(ref player) = connect.player {
        for rank in 0..RANKS {
            for file in 0..FILES {
                let pos = Position::new(rank, file);
                let piece = board.board.get(pos);
                let mut translation = locate_piece(pos, player.color);
                commands.spawn((
                    PieceMarker,
                    pos,
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(*PIECE_EACH, *PIECE_EACH)),
                            ..Default::default()
                        },
                        texture: if piece.is_color(PieceColor::Red) {
                            red.0.get(&piece.kind()).unwrap().clone()
                        } else {
                            black.0.get(&piece.kind()).unwrap().clone()
                        },
                        transform: Transform::from_translation(translation),
                        ..Default::default()
                    },
                ));
                translation.z = -2.0;
                commands.spawn((
                    TileMarker,
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(*PIECE_EACH, *PIECE_EACH)),
                            ..Default::default()
                        },
                        texture: tile.0.clone(),
                        transform: Transform::from_translation(translation),
                        ..Default::default()
                    },
                ));
            }
        }
    }
}

pub(super) fn update_pieces(
    mut update: EventReader<UpdateEvent>,
    mut q_pieces: Query<(&Position, &mut Handle<Image>), With<PieceMarker>>,
    board: Res<BoardInfo>,
    red: Res<RedImages>,
    black: Res<BlackImages>,
) {
    update.read().for_each(|_| {
        q_pieces.iter_mut().for_each(|(pos, mut image)| {
            let piece = board.board.get(*pos);
            *image = if piece.is_color(PieceColor::Red) {
                &red.0
            } else {
                &black.0
            }
            .get(&piece.kind())
            .unwrap()
            .clone();
        });
    });
}

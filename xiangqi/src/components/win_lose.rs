use super::*;

#[derive(Component)]
pub struct WinLoseMarker;

#[derive(Resource)]
pub struct GameJustStarted(pub bool);

impl Default for GameJustStarted {
    fn default() -> Self {
        Self(true)
    }
}

pub(super) fn listen_win_lose(
    mut commands: Commands,
    mut event: EventReader<UpdateEvent>,
    moves: Res<Moves>,
    board: Res<BoardInfo>,
    font: Res<DefaultFont>,
    connect: Res<Connection>,
    win_lose: Query<(), With<WinLoseMarker>>,
    mut started: ResMut<GameJustStarted>,
) {
    if let Some(ref player) = connect.player {
        event.read().for_each(|_| {
            if !started.0 && win_lose.iter().next().is_none() && moves.moves.is_empty() {
                let (text, color) = if board.board.turn() == player.color {
                    ("You Lose!", Color::RED)
                } else {
                    ("You Win!", Color::GREEN)
                };
                commands.spawn((
                    WinLoseMarker,
                    Text2dBundle {
                        text: Text::from_section(
                            text,
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 70.0,
                                color,
                            },
                        ),
                        ..Default::default()
                    },
                ));
            }
        });
    }
    started.0 = false;
}

pub(super) fn despawn_win_lose(
    mut commands: Commands,
    win_lose: Query<Entity, With<WinLoseMarker>>,
) {
    win_lose.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    })
}

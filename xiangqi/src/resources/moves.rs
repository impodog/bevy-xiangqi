use super::*;

#[derive(Debug, Default, Resource)]
pub struct Moves {
    pub moves: HashMap<Position, HashSet<Position>>,
}

#[derive(Debug, Clone, Event)]
pub struct UpdateEvent;

pub(super) fn init_moves(mut commands: Commands) {
    commands.init_resource::<Moves>();
}

pub(super) fn listen_update(
    mut update: EventReader<UpdateEvent>,
    board: Res<BoardInfo>,
    mut moves: ResMut<Moves>,
) {
    update.read().for_each(|_| {
        let map = Arc::new(RwLock::new(HashMap::<Position, HashSet<Position>>::new()));
        let pool = threadpool::Builder::new().build();
        for rank in 0..RANKS {
            for file in 0..FILES {
                let pos = Position::new(rank, file);
                if board.board.get(pos).is_color(board.board.turn()) {
                    let reachable = board.board.reachable(pos);
                    for to in reachable.into_iter() {
                        let mut board = board.board.clone();
                        let map = map.clone();
                        pool.execute(move || {
                            board.force(pos, to);
                            if !board.is_check() {
                                map.write().unwrap().entry(pos).or_default().insert(to);
                            }
                        });
                    }
                }
            }
        }
        pool.join();
        moves.moves = RwLock::into_inner(Arc::into_inner(map).unwrap()).unwrap();
    });
}

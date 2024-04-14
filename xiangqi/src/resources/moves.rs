use super::*;

#[derive(Debug, Default, Resource)]
pub struct Moves {
    pub moves: HashMap<Position, HashSet<Position>>,
}

#[derive(Debug, Clone, Event)]
pub struct UpdateEvent;

#[derive(Debug, Clone, Event)]
pub struct TryMoveEvent {
    pub from: Position,
    pub to: Position,
}

#[derive(Debug, Clone, Event)]
pub struct DoMoveEvent {
    pub from: Position,
    pub to: Position,
}

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
        info!("Updated available moves")
    });
}

pub(super) fn verify_move(
    mut try_move: EventReader<TryMoveEvent>,
    mut do_move: EventWriter<DoMoveEvent>,
    board: Res<BoardInfo>,
    connect: Res<Connection>,
    moves: Res<Moves>,
) {
    try_move.read().for_each(|mv| {
        info!("Move request: {:?}", mv);
        if connect.is_connected()
            && board.board.turn() == connect.player.as_ref().unwrap().color
            && mv.from.legal().is_some()
            && mv.to.legal().is_some()
            && moves
                .moves
                .get(&mv.from)
                .map(|set| set.contains(&mv.to))
                .unwrap_or_default()
        {
            do_move.send(DoMoveEvent {
                from: mv.from,
                to: mv.to,
            });
            info!("Request sent");
        } else {
            info!("Request denied");
        }
    });
}

pub(super) fn do_move(
    mut do_move: EventReader<DoMoveEvent>,
    mut update: EventWriter<UpdateEvent>,
    mut board: ResMut<BoardInfo>,
    _connect: Res<Connection>,
) {
    do_move.read().for_each(|mv| {
        board.board.force(mv.from, mv.to);
        board.board.next_turn();
        // todo Do something with the connection
        update.send(UpdateEvent);
    });
}

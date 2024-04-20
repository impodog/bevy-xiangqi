use super::*;

pub struct QueryTimer(pub Timer);

impl Default for QueryTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

pub(super) fn query_moves(
    mut request: EventWriter<TypedRequest<QueryResponse>>,
    connect: Res<Connection>,
    time: Res<Time>,
    mut timer: Local<QueryTimer>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        if let Some(ref player) = connect.player {
            //info!("Querying player moves");
            let body = QueryRequest {
                room: connect.room,
                player: player.color.into(),
            };
            request.send(
                HttpClient::new()
                    .json(&body)
                    .get(format!("{}/query", connect.url))
                    .with_type(),
            );
        }
    }
}

pub(super) fn respond_moves(
    mut response: EventReader<TypedResponse<QueryResponse>>,
    mut update: EventWriter<UpdateEvent>,
    mut board: ResMut<BoardInfo>,
) {
    response.read().for_each(|response| {
        if let Some(ref board_s) = response.board {
            info!("Updating board to {:?}", board_s);
            board.board = board_s.as_str().try_into().unwrap();
            update.send(UpdateEvent);
        } else {
            // Continue to wait
        }
    });
}

use super::*;

#[derive(Debug, Clone)]
pub struct Player {
    pub color: PieceColor,
}

#[derive(Debug, Clone, Default, Resource)]
pub struct Connection {
    pub url: String,
    pub room: RoomId,
    pub player: Option<Player>,
}

#[derive(Debug, Event)]
pub struct ConnectEvent;

impl Connection {
    pub fn is_connected(&self) -> bool {
        self.player.is_some()
    }
}

pub(super) fn listen_connect_event(
    mut connections: EventReader<ConnectEvent>,
    mut request: EventWriter<TypedRequest<ConnectResponse>>,
    connect: Res<Connection>,
) {
    connections.read().for_each(|_| {
        let body = ConnectRequest { room: connect.room };
        info!(
            "Attempting to connect: {:?} in {}",
            connect.url, connect.room
        );
        request.send(
            HttpClient::new()
                .json(&body)
                .post(format!("{}/connect", connect.url))
                .with_type(),
        );
    });
}

pub(super) fn update_connection_player(
    mut response: EventReader<TypedResponse<ConnectResponse>>,
    mut connect: ResMut<Connection>,
) {
    response.read().for_each(|response| {
        if response.ok {
            info!(
                "Connected as {:?}",
                Into::<PieceColor>::into(response.player)
            );
            connect.player = Some(Player {
                color: response.player.into(),
            })
        } else {
            warn!("Connection failed");
        }
    });
}

pub(super) fn init_connection(mut commands: Commands) {
    commands.insert_resource(Connection::default());
}

pub(super) fn test_disconnect(
    mut status: ResMut<NextState<Status>>,
    mut request: EventWriter<HttpRequest>,
    connect: Res<Connection>,
) {
    if connect.player.is_none() {
        status.set(Status::Menu);
        let body = DisconnectRequest { room: connect.room };
        request.send(
            HttpClient::new()
                .json(&body)
                .post(format!("{}/disconnect", connect.url))
                .build(),
        );
    }
}

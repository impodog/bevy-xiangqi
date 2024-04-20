use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::*;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use transfer::*;

static EXPIRE: std::time::Duration = std::time::Duration::from_secs(20);

#[derive(Debug, Clone, Copy)]
enum RoomStatus {
    One(bool),
    Full,
}

impl Default for RoomStatus {
    fn default() -> Self {
        Self::One(rand::random())
    }
}

#[derive(Default)]
struct Room {
    next: RoomStatus,
    left: VecDeque<String>,
    right: VecDeque<String>,
    last: Duration,
}

impl Room {
    fn get(&mut self, player: bool) -> &mut VecDeque<String> {
        if player {
            &mut self.right
        } else {
            &mut self.left
        }
    }

    fn update(&mut self) {
        self.last = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    }

    fn is_expired(&self) -> bool {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let diff = now - self.last;
        diff > EXPIRE
    }
}

struct Games {
    rooms: Arc<RwLock<HashMap<RoomId, Room>>>,
}

#[post("/connect", data = "<req>")]
fn connect(
    games: &State<Games>,
    req: Json<ConnectRequest>,
) -> status::Custom<Json<ConnectResponse>> {
    let response = match games.rooms.write().unwrap().entry(req.room) {
        Entry::Occupied(occupied) => {
            let room = occupied.into_mut();
            room.update();

            let response = match room.next {
                RoomStatus::One(player) => {
                    info!("Player joined room {}", req.room);
                    Some(ConnectResponse { player, ok: true })
                }
                RoomStatus::Full => {
                    warn!("Room {} is full, joining rejected", req.room);
                    None
                }
            };
            room.next = RoomStatus::Full;
            response
        }
        Entry::Vacant(vacant) => {
            let room = vacant.insert(Room::default());
            room.update();

            if let RoomStatus::One(player) = room.next {
                info!("New room {} created, player joined", req.room);
                let player = !player;
                Some(ConnectResponse { player, ok: true })
            } else {
                unreachable!()
            }
        }
    };
    match response {
        Some(response) => status::Custom(Status::Ok, Json(response)),
        None => status::Custom(Status::Conflict, Json(ConnectResponse::default())),
    }
}

#[post("/play", data = "<req>")]
fn play(games: &State<Games>, req: Json<PlayRequest>) -> Status {
    if let Some(room) = games.rooms.write().unwrap().get_mut(&req.room) {
        room.update();

        // Here player is flipped because the move is only effective on the opponent side
        let player = !req.player;
        room.get(player).push_back(req.board.clone());
        info!("Update room {} to {:?}", req.room, req.board);
        Status::Accepted
    } else {
        warn!("Update room {} was rejected", req.room);
        Status::ServiceUnavailable
    }
}

#[get("/query", data = "<req>")]
fn query(games: &State<Games>, req: Json<QueryRequest>) -> Json<QueryResponse> {
    let board = if let Some(room) = games.rooms.write().unwrap().get_mut(&req.room) {
        room.update();

        info!("Query room {}", req.room);
        room.get(req.player).pop_front()
    } else {
        info!("Query room {} was rejected", req.room);
        None
    };
    Json(QueryResponse { board })
}

#[post("/disconnect", data = "<req>")]
fn disconnect(games: &State<Games>, req: Json<DisconnectRequest>) {
    info!("Disconnect from room {}", req.room);
    games.rooms.write().unwrap().remove(&req.room);
}

#[launch]
fn rocket() -> _ {
    let rooms = Arc::new(RwLock::new(HashMap::<RoomId, Room>::new()));
    let rooms_clone = rooms.clone();

    std::thread::spawn(move || loop {
        let mut destruct = Vec::new();
        rooms_clone.write().unwrap().iter().for_each(|(id, room)| {
            if room.is_expired() {
                destruct.push(*id);
            }
        });
        for id in destruct.into_iter() {
            rooms_clone.write().unwrap().remove(&id);
        }
        std::thread::sleep(EXPIRE);
    });

    rocket::build()
        .manage(Games { rooms })
        .mount("/", routes![connect, play, query, disconnect])
}

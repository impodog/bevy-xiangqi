use super::*;

#[derive(Resource)]
pub struct BlackImages(pub HashMap<PieceKind, Handle<Image>>);

#[derive(Resource)]
pub struct RedImages(pub HashMap<PieceKind, Handle<Image>>);

#[derive(Resource)]
pub struct MarkerImage(pub Handle<Image>);

#[derive(Resource)]
pub struct TileImage(pub Handle<Image>);

pub(super) fn init_images(mut commands: Commands, server: Res<AssetServer>) {
    for color in ['r', 'b'] {
        let mut map = HashMap::new();
        for piece in ['p', 'c', 'k', 'a', 'b', 'n', 'r', '-'] {
            let kind = piece.try_into().unwrap();
            map.insert(kind, server.load(format!("{}/{}.png", color, piece)));
        }
        match color {
            'r' => {
                commands.insert_resource(RedImages(map));
            }
            'b' => {
                commands.insert_resource(BlackImages(map));
            }
            _ => unreachable!(),
        }
    }
    commands.insert_resource(MarkerImage(server.load("marker.png")));
    commands.insert_resource(TileImage(server.load("tile.png")));
}

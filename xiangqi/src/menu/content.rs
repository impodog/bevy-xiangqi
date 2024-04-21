use super::*;

#[derive(Debug, Event)]
pub struct LaunchEvent;

#[derive(Debug, Resource)]
pub struct MenuContents {
    pub url: String,
    pub room: String,
}

impl Default for MenuContents {
    fn default() -> Self {
        Self {
            url: "http://127.0.0.1:8082".to_string(),
            room: "example-room-code".to_string(),
        }
    }
}

pub(super) fn init_contents(mut commands: Commands) {
    commands.init_resource::<MenuContents>();
}

pub(super) fn init_ui(mut contexts: Query<&mut bevy_egui::EguiContext>) {
    contexts
        .iter_mut()
        .for_each(|mut context| context.get_mut().set_pixels_per_point(5.0));
}

pub(super) fn menu_ui(
    mut contexts: EguiContexts,
    mut contents: ResMut<MenuContents>,
    mut launch: EventWriter<LaunchEvent>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.label("Connection URL:");
        ui.text_edit_singleline(&mut contents.url);
        ui.label("Room Code:");
        ui.text_edit_singleline(&mut contents.room);
        if ui.button("Connect").clicked() {
            launch.send(LaunchEvent);
        }
    });
}

pub(super) fn launch_game(
    mut launch: EventReader<LaunchEvent>,
    mut connect: ResMut<Connection>,
    contents: Res<MenuContents>,
    mut event: EventWriter<ConnectEvent>,
) {
    launch.read().for_each(|_| {
        connect.url = contents.url.clone();
        let mut value = 0u64;
        let mut base = 1u64;
        for c in contents.room.chars() {
            value = value
                .overflowing_add((c as u32 as u64).overflowing_mul(base).0)
                .0;
            base = base.overflowing_mul(131).0;
        }
        connect.room = value;
        event.send(ConnectEvent);
    });
}

pub(super) fn boot_game(
    mut commands: Commands,
    connect: Res<Connection>,
    mut status: ResMut<NextState<Status>>,
) {
    if connect.player.is_some() {
        // This is NOT init_resource. It overrides the previous value.
        commands.insert_resource(GameJustStarted::default());
        status.set(Status::Play);
    }
}

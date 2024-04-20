use ::xiangqi::prelude::*;
use bevy::prelude::*;

#[cfg(not(debug_assertions))]
fn debug_connect() {}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: bevy::window::WindowResolution::new(WIDTH, HEIGHT)
                        .with_scale_factor_override(1.0),
                    title: "Xiangqi".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .add_plugins((bevy_http_client::HttpClientPlugin, bevy_egui::EguiPlugin))
        .add_plugins((ComponentsPlugin, ResourcesPlugin, StatusPlugin, MenuPlugin))
        .run();
}

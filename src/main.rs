use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use turn_based::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Turn Based Game".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .add_plugins(EditorPlugin::default())
        .run();
}

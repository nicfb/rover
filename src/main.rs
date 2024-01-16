use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod world;

use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, WorldPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}

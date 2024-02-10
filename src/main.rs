use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


use starfield_simulation::StarfieldSimulationPlugin;


mod starfield_simulation;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(StarfieldSimulationPlugin)
        .run();
}
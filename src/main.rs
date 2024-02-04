use bevy::prelude::*;
use starfield_simulation::StarfieldSimulationPlugin;

mod starfield_simulation;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StarfieldSimulationPlugin)
        .run();
}
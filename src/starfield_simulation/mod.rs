use bevy::prelude::*;
use nannou::rand::random_range;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;
const POPULATION: u32 = 100;

pub struct StarfieldSimulationPlugin;

pub struct Star {
    x: f32,
    y: f32,
    z: f32,
}



impl Star {
    fn new() -> Self {
        Star {
            // x: random_range(-(WIDTH as f32) / 2.0, WIDTH as f32 / 2.0),
            // y: random_range(-(HEIGHT as f32) / 2.0, HEIGHT as f32 / 2.0),
            // z: random_range(0.0, WIDTH as f32 / 2.0),
            x: random_range(-(WIDTH as f32), WIDTH as f32),
            y: random_range(-(HEIGHT as f32), HEIGHT as f32),
            // z: random_range(0.0, WIDTH as f32),
            z: WIDTH as f32,
        }
    }
}

#[derive(Resource, Default)]
pub struct Model {
    stars: Vec<Star>,
}


impl Plugin for StarfieldSimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Model::default())
            .add_systems(Startup, setup);
    }
}

fn setup(mut model: ResMut<Model>) {
    println!("Starfield simulation setup");
    let mut stars = Vec::new();
    for _ in 0..POPULATION {
        stars.push(Star::new());
    }

    model.stars = stars;
}
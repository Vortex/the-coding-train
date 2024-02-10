use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use nannou::{draw, rand::random_range};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;
const POPULATION: u32 = 100;

pub struct StarfieldSimulationPlugin;

#[derive(Component, Debug)]
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

impl Plugin for StarfieldSimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Startup, spawn_stars.after(setup));
            // .add_systems(Update, (update_positions, draw_stars));
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());

    println!("Starfield simulation setup");
    
    // for _ in 0..POPULATION {
    //     stars.push(Star::new());
    // }



    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });    
}

// fn update_positions(mut model: ResMut<Model>) {
//     for i in 0..POPULATION as usize {
//         let star = &mut model.stars[i];
//         star.y = star.y + 1.0;
//     }
// }

fn spawn_stars(
    mut commands: Commands,
    // query: Query<&Star>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut stars = Vec::new();
    for _ in 0..POPULATION {
        stars.push(Star::new());
    }

    for star in stars {
        println!("Star: {:?}", star);
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(8.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(star.x, star.y, 0.1)),
            ..default()
        });
    }
}

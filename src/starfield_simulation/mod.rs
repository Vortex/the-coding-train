use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use nannou::{math::map_range, rand::random_range};

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
        app.add_systems(Startup, setup)
            .add_systems(Startup, spawn_stars.after(setup))
            .add_systems(Update, move_stars);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });
}

fn spawn_stars(
    mut commands: Commands,   
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in 0..POPULATION {
        let star = Star::new();        
        println!("Star: {:?}", star);

        // Use this so calculate the precise position of the star
        let sx: f32 = map_range(star.x / star.z, 0.0, 1.0, 0.0, WIDTH as f32);
        let sy = map_range(star.y / star.z, 0.0, 1.0, 0.0, HEIGHT as f32);        

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(8.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(sx, sy, 0.1)),
                ..default()
            })
            .insert(star);
    }
}

fn move_stars(mut query: Query<(&mut Star, &mut Transform)>, time: Res<Time>) {
    for (mut star, mut transform) in query.iter_mut() {
        let delta = time.delta_seconds() * 50.0; // Movement speed

        // Update Star component
        star.z -= delta;

        if star.z < 1.0 {
            star.z = WIDTH as f32;
            star.x = random_range(-(WIDTH as f32), WIDTH as f32);
            star.y = random_range(-(HEIGHT as f32), HEIGHT as f32);
        }

        let sx: f32 = map_range(star.x / star.z, 0.0, 1.0, 0.0, WIDTH as f32);
        let sy = map_range(star.y / star.z, 0.0, 1.0, 0.0, HEIGHT as f32);        


        // Apply the updated position to the Transform component
        *transform = Transform::from_xyz(sx, sy, 0.1);
    }
}

use nannou::prelude::*;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;
const POPULATION: u32 = 100;

fn main() {
    nannou::app(model).update(update).run();
}

// #[derive(Copy)]
struct Star {
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
struct Model {
    stars: Vec<Star>,
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    let mut stars = Vec::new();
    for _ in 0..POPULATION {
        stars.push(Star::new());
    }

    Model { stars, _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Update code here...
    for i in 0..POPULATION as usize {
        let star = &mut _model.stars[i];
        star.z = star.z + 1.0;
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to black.
    draw.background().color(BLACK);

    // Draw stars
    for i in 0..POPULATION as usize {
        let star = &_model.stars[i];

        let sx = map_range(star.x / star.z, 0.0, 1.0, 0.0, WIDTH as f32);
        let sy = map_range(star.y / star.z, 0.0, 1.0, 0.0, HEIGHT as f32);

        draw.ellipse()
            .x_y(sx, sy)
            .w_h(8.0, 8.0)
            .color(WHITE)
            .stroke(WHITE);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

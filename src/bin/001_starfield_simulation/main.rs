use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(512, 512).view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Update code here...
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(BLUE);

    // Draw a white ellipse to follow the mouse.
    draw.ellipse().x_y(0.0, 0.0).w_h(100.0, 100.0).color(WHITE);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

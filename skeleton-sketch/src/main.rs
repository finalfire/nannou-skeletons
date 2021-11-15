use nannou::prelude::*;

// Draws on the screen
fn view(app: &App, frame: Frame) {
    // We will use `draw` to draw on screen
    let draw = app.draw();
    
    // Let's first color the background
    draw.background().color(WHITE);
    // then draw a 42 x 42 (pixels) orange rectangle
    // at the center of the screen (0, 0)
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(42.0, 42.0)
        .color(ORANGE);
    
    // Eventually, we write our `draw` to frame
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::sketch(view)
        .size(720, 720)    
        .run();
}
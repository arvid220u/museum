use nannou::prelude::*;

pub fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    dots: Vec<Vector2>
}

const NUM_DOTS: usize = 1000;
const DOT_RADIUS: f32 = 3.0;
const WINDOW_SIZE: u32 = 700;

fn model(app: &App) -> Model {
    let _window = app.new_window().size(WINDOW_SIZE,WINDOW_SIZE).view(view).build().unwrap();

    let mut dots = Vec::new();

    for _ in 0..NUM_DOTS {
        let dot = vec2(
            (random::<f32>()-0.5)*(WINDOW_SIZE as f32),
            (random::<f32>()-0.5)*(WINDOW_SIZE as f32),
        );
        dots.push(dot);
    }

    Model {
        dots
    }

}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let rand_index = ((random::<f32>()) * (NUM_DOTS as f32)).floor() as usize;
    let mut rand_dot = &mut model.dots[rand_index];
    *rand_dot = vec2(rand_dot.x + 1.0, rand_dot.y + 1.0);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for dot in &model.dots {
        draw.ellipse().x_y(dot.x, dot.y).radius(DOT_RADIUS).color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

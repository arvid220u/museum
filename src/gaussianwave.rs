use nannou::prelude::*;
use crate::paper;

pub fn main() {
    nannou::sketch(view).run();
}

const DOT_RADIUS: f32 = 3.0;
const NUM_Y: usize = 30;
const NUM_X: usize = 50;
const BACKGROUND_COLOR: Srgb<u8> = BLACK;
const DOT_COLOR: Srgb<u8> = WHITE;

// TODO: make the bounces smooth somehow
// TODO: make time dependency smooth / sine wave?
// TODO: use some 2D function instead of 1D, or make y dimension interesting somehow

fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    let t = app.elapsed_frames();

    // set background to blue
    draw.background().color(BACKGROUND_COLOR);

    let del_y = win.h() / (NUM_Y as f32);
    // let del_x = (win.right() - win.left()) / (NUM_X as f32);
    let l = 3.0;
    let f = gaussian_fp(t, 0.02, l/10.0, l-l/10.0, 3.0);
    let mut xs = paper::project(f, l, NUM_X, 0.0001);
    // scale the xs
    for x in &mut xs {
        *x = (*x / l) * (win.w() as f64) + (win.left() as f64);
    }
    for yi in 0..NUM_Y {
        let y = win.bottom() + del_y / 2.0 + del_y * (yi as f32);
        // for xi in 0..NUM_X {
        //     // let x = win.left() + del_x / 2.0 + del_x * (xi as f32);
        //     draw.ellipse().x_y(x, y).radius(DOT_RADIUS).color(WHITE);
        // }
        for x in &xs {
            draw.ellipse().x_y(*x as f32, y).radius(DOT_RADIUS).color(DOT_COLOR);
        }
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

// gaussian travelling in [a,b] interval
fn gaussian_fp(t: u64, speed: f64, a: f64, b: f64, mag: f64) -> impl Fn(f64) -> f64 {
    let mut mu = ((t as f64) * speed) % (2.0*(b-a)) + a;
    if mu > b {
        mu -= 2.0 * (mu - b);
    }
    move |x| {
        - mag * (x - mu) * (-(x-mu).powi(2)).exp()
    }
}

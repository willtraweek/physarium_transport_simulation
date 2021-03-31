extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod dish;
mod cell;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::EventLoop;

const MAX_FPS: u64 = 5;
const WINDOW_WIDTH: u32 = 200;
const WINDOW_HEIGHT: u32 = 400;
const BOX_SIZE: f64 = WINDOW_HEIGHT as f64 / 2.0;


fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Physarium Transport Simulation", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut dish = dish::Dish::new(WINDOW_WIDTH, WINDOW_HEIGHT, opengl);

    let mut events = Events::new(EventSettings::new());
    events.set_ups(MAX_FPS + 1);
    events.set_max_fps(MAX_FPS + 1);

    let mut start = time::Instant::now();
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            dish.render(&args);
            let stop = time::Instant::now();
            println!("full render: {} ms", (stop-start).as_millis())
        }
        start = time::Instant::now();

        if let Some(args) = e.update_args() {
            dish.update(&args);
        }
    }
}
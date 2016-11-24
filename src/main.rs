#![feature(cfg_target_feature)]
#![feature(test)]

#[macro_use] extern crate itertools;
extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate simd;
extern crate test;
mod app;
mod bodies;


use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("N-body sim", [900, 900])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = app::App {
        gl: GlGraphics::new(opengl),
        bodies: bodies::sample(),
        mass_to_display_factor: 25.0f64,
        frames_to_draw: 1000000000,
        frames_drawn: 0,
    };
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) { 
        if let Some(r) = e.render_args() {
            app.render(&r);
        } else if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }


}

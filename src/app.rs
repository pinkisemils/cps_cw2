use std::sync::{Arc,Mutex};
use opengl_graphics::GlGraphics;
use piston_window::{UpdateArgs, RenderArgs};
use bodies::{Body, SimpleBody};


pub struct App {
    pub gl: GlGraphics,
    pub bodies: Arc<Mutex<Box<Vec<SimpleBody>>>>,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        println!("Render event");

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let bodies = self.bodies.clone();
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            {
                let bodies = bodies.lock().unwrap();
                for body in (*bodies).iter() {
                    let (bcx, bcy) = body.coordinates();
                    let circle = rectangle::square(bcx, bcy, 10.0f64);

                    ellipse(BLACK, circle, c.transform, gl);
                }

            }

        });
    }

    pub fn update(&mut self, _: &UpdateArgs) {
        println!("Update event");
    }
}
use std::sync::{Arc,RwLock};
use opengl_graphics::GlGraphics;
use piston_window::{UpdateArgs, RenderArgs};
use bodies::{Body, SimpleBody, advance};


pub struct App{
    pub gl: GlGraphics,
    //pub bodies: Arc<RwLock<Box<Vec<SimpleBody>>>>,
    pub bodies: Vec<SimpleBody>,
    pub mass_to_display_factor: f64,
    pub frames_to_draw: u64,
    pub frames_drawn: u64,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        //println!("Render event");

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let bodies = self.bodies.clone();
        let mass_factor = self.mass_to_display_factor;
        let ref bodies = self.bodies;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            {
                for body in bodies.iter() {
                    let (bcx, bcy) = body.coordinates();
                    let circle = rectangle::square(bcx, bcy, 10.0);

                    ellipse(BLACK, circle, c.transform, gl);
                }

            }

        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
       if self.frames_to_draw > self.frames_drawn {
            self.bodies = advance(&self.bodies, args.dt * 10000000.0);
            println!("Update event");
            self.frames_drawn += 1;
        }
    }
}

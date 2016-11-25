use std::sync::{Arc,RwLock};
use opengl_graphics::GlGraphics;
use piston_window::{UpdateArgs, RenderArgs};
use bodies::{Body, SimpleBody, advance};
use std::cmp::max;


pub struct App{
    pub gl: GlGraphics,
    //pub bodies: Arc<RwLock<Box<Vec<SimpleBody>>>>,
    pub bodies: Vec<SimpleBody>,
    pub mass_to_display_factor: f64,
    pub frames_to_draw: u64,
    pub frames_drawn: u64,
    pub timestep: f64,
    pub timestep_factor: f64,
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
        let mut itr = 0;
        let max_itrs = f64::ceil(args.dt / self.timestep * self.timestep_factor) as u64;
                           

       while (self.frames_to_draw > self.frames_drawn  && max_itrs > itr) {
            self.bodies = advance(&self.bodies, self.timestep);
            println!("Update event");
            self.frames_drawn += 1;
            itr +=1;
        }
    }
}

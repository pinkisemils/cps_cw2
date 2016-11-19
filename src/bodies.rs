/// An interface and a mock struct to make bodies drawable
///
use std::sync::{Arc,Mutex};
use std::clone::Clone;
pub trait Body {
    fn coordinates(&self) -> (f64, f64) where Self: Sized + Clone;
    fn mass(&self) -> f64 where Self: Sized + Clone;
}

pub trait DrawableBodies {
    fn bodies(&self) -> Vec<Box<Body>> where Self: Sized;
}

#[derive(Clone)]
pub struct SimpleBody {
    x: f64,
    y: f64,
    mass: f64,
    energy: f64,
}

impl Body for SimpleBody {
    fn coordinates(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn mass(&self) -> f64 {
        self.mass
    }
}


fn _sample_bodies() -> Vec<SimpleBody> {
    vec![SimpleBody{x:20.0, y:20.0, mass:100.0, energy: 300.0},
        SimpleBody{x:120.0, y:120.0, mass:100.0, energy: 300.0},
        SimpleBody{x:20.0, y:120.0, mass:100.0, energy: 300.0},
        SimpleBody{x:120.0, y:20.0, mass:100.0, energy: 300.0},
        ]
}

pub fn sample_bodies() -> Arc<Mutex<Box<Vec<SimpleBody>>>> {
    Arc::new(Mutex::new(Box::new(_sample_bodies())))


}

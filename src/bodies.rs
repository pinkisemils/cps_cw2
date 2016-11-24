/// An interface and a mock struct to make bodies drawable
///
#[cfg(target_feature = "sse2")]
use simd::x86::sse2;

use std::sync::{Arc, RwLock};
use std::clone::Clone;
pub trait Body {
    fn coordinates(&self) -> (f64, f64) where Self: Sized + Clone;
    fn mass(&self) -> f64 where Self: Sized + Clone;
}

// pub trait Bodies<K> where K: Body {
//    fn bodies(&self)<'a> -> 'a K;
// }

pub trait DrawableBodies {
    fn bodies(&self) -> Vec<Box<Body>> where Self: Sized;
}

#[derive(Clone)]
pub struct SimpleBody {
    pos: sse2::f64x2,
    velocity: sse2::f64x2,
    mass: f64,
}


static EPS: f64 = 1.5e1 * 1.5e1;

pub fn advance(previous: &Vec<SimpleBody>, dt: f64) -> Vec<SimpleBody> {
    let len_bodies = previous.len();
    //let mut output: Vec<SimpleBody> = Vec::with_capacity(len_bodies);
    let mut output: Vec<SimpleBody> = previous.clone();

    let mut forces: Vec<Vec<sse2::f64x2>> = Vec::with_capacity(len_bodies);
    let dt_mult = sse2::f64x2::splat(dt);

    for (i, cur_bod) in previous.iter().enumerate() {
        let cur_bod_mass = sse2::f64x2::splat(cur_bod.mass);
        let mut accel = [0.0f64; 2];
        forces.push(Vec::with_capacity(len_bodies));
        for (j, bod) in previous.iter().skip(i).enumerate() {
            let bod_mass = sse2::f64x2::splat(bod.mass);
            let dist = cur_bod.pos - bod.pos;
            let dist = dist * dist;
            let dist_sq = dist.extract(0) + dist.extract(1);
            let force = sse2::f64x2::splat((G * cur_bod.mass * bod.mass) / (dist_sq + EPS)); 
            let angle = dist.extract(1).atan2(dist.extract(0));

            let forces = sse2::f64x2::new(angle.cos(), angle.sin()) * force;
                
            
            output[i].velocity = output[i].velocity + (forces / cur_bod_mass);
            output[i + j].velocity = output[i+j].velocity + (forces / bod_mass);
        }
    }

    for ref mut bod in output.iter_mut() {
        bod.pos = bod.pos +  (bod.velocity * dt_mult);
    }

    output
}

impl Body for SimpleBody {
    fn coordinates(&self) -> (f64, f64) {
        (self.pos.extract(0),self.pos.extract(1))
    }

    fn mass(&self) -> f64 {
        self.mass
    }
}


fn _sample_bodies() -> Vec<SimpleBody> {
    vec![SimpleBody{pos:sse2::f64x2::new(20.0, 20.0), velocity:sse2::f64x2::new(0.0, 0.0), mass:100.0},
        SimpleBody{pos:sse2::f64x2::new(120.0, 120.0), velocity:sse2::f64x2::new(0.0, 0.0), mass:100.0},
        SimpleBody{pos:sse2::f64x2::new(20.0, 120.0), velocity:sse2::f64x2::new(0.0, 0.0), mass:100.0},
        SimpleBody{pos:sse2::f64x2::new(120.0, 20.0), velocity:sse2::f64x2::new(0.0, 0.0), mass:100.0},
        ]
}

pub fn sample_bodies() -> Arc<RwLock<Box<Vec<SimpleBody>>>> {
    Arc::new(RwLock::new(Box::new(_sample_bodies())))
}

static G: f64 = 6.67834 * 1e-11;
static Damp: f64 = 1.5e1;

/// An interface and a mock struct to make bodies drawable
///
#[cfg(target_feature = "sse2")]
use simd::x86::sse2::*;

use std::sync::{Arc, RwLock};
use std::clone::Clone;
use std::cmp::max;

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
    pos: f64x2,
    velocity: f64x2,
    mass: f64,
}


static EPS: f64 = 1.5e1 * 1.5e1;
static G: f64 = 6.67834 * 1e-11;
static Damp: f64 = 1.5e1;

pub fn advance(previous: &Vec<SimpleBody>, dt: f64) -> Vec<SimpleBody> {
    let len_bodies = previous.len();
    //let mut output: Vec<SimpleBody> = Vec::with_capacity(len_bodies);
    let mut output: Vec<SimpleBody> = previous.clone();
    let damp: f64x2 = f64x2::splat(0.9995f64);
    let undamp: f64x2 = f64x2::splat(0.5f64); 
    let min = f64x2::splat(1e-10);
                                   

    let mut forces: Vec<Vec<f64x2>> = Vec::with_capacity(len_bodies);
    let dt_mult = f64x2::splat(dt);

    for (i, cur_bod) in previous.iter().enumerate() {
        let cur_bod_mass = f64x2::splat(cur_bod.mass);
        let mut accel = [0.0f64; 2];
        forces.push(Vec::with_capacity(len_bodies));
        for (j, bod) in previous.iter().skip(i+1).enumerate() {
            let bod_mass = f64x2::splat(bod.mass);
            let dist = cur_bod.pos - bod.pos;
            let angle = dist.extract(1).atan2(dist.extract(0));
            let dist = dist * dist;
            let mut dist_sq = (dist.extract(0) + dist.extract(1));
            

            let force = (G * cur_bod.mass * bod.mass); 

            let forces = f64x2::new(angle.cos(), angle.sin()) 
                       * f64x2::splat(force)  
                       / f64x2::splat((dist_sq + EPS));
           
            output[i].velocity = output[i].velocity - (forces / cur_bod_mass);
            output[i+j+1].velocity = output[i+j+1].velocity + (forces / bod_mass);
        }
    }

    for ref mut bod in output.iter_mut() {
        bod.pos = bod.pos +  (bod.velocity * dt_mult);
    }
    println!("Previous coords: {:?}", previous[0].pos);
    println!("Next coords: {:?}", output[0].pos);

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

impl SimpleBody {
    pub fn new(x:f64, y:f64, vx:f64, vy:f64, mass:f64) -> SimpleBody {
                SimpleBody{pos:f64x2::new(x, y),
                            velocity:f64x2::new(vx, vy),
                            mass:mass}

    }
}


fn _sample_bodies() -> Vec<SimpleBody> {
    let default_mass = 1000000000.0f64;
    let rect = [500.0, 400.0];
    vec![SimpleBody::new(rect[0], rect[0], 0.0, 0.0, default_mass),
        SimpleBody::new(rect[0], rect[1], 0.0, 0.0, default_mass),
        SimpleBody::new(rect[1], rect[0], 0.0, 0.0, default_mass),
        SimpleBody::new(rect[1], rect[1], 0.0, 0.0, default_mass),
        ]
}

pub fn sample_bodies() -> Arc<RwLock<Box<Vec<SimpleBody>>>> {
    Arc::new(RwLock::new(Box::new(_sample_bodies())))
}
pub fn sample() -> Vec<SimpleBody> {
    _sample_bodies()

}


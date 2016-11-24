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
    //accel: sse2::f64x2,
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
        for (j, bod) in previous.iter().skip(i+1).enumerate() {
            let bod_mass = sse2::f64x2::splat(bod.mass);
            let dist = cur_bod.pos - bod.pos;
            let angle = dist.extract(1).atan2(dist.extract(0));
            let dist = dist * dist;
            let dist_sq = dist.extract(0) + dist.extract(1);
            for _ in 0..3 {
                dist_sq = 
            }
            let force = sse2::f64x2::splat((G * cur_bod.mass * bod.mass) / (dist_sq + EPS)); 

            let forces = sse2::f64x2::new(angle.cos(), angle.sin()) * force;
            
            output[i].velocity = output[i].velocity - (forces / cur_bod_mass);
            //output[i].accel = (forces / cur_bod_mass);
            output[i+j+1].velocity = output[i+j+1].velocity + (forces / bod_mass);
            //output[i+j].accel = (forces / cur_bod_mass);
            //println!("I: {}", i);
            //println!("I+J: {}", i+j+1);
            //println!("J: {}", j);
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
                SimpleBody{pos:sse2::f64x2::new(x, y),
                            velocity:sse2::f64x2::new(vx, vy),
                            mass:mass}

    }
}


fn _sample_bodies() -> Vec<SimpleBody> {
    let default_mass = 1000000.0f64;
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

static G: f64 = 6.67834 * 1e-11;
static Damp: f64 = 1.5e1;

use nalgebra::Unit;
use rand::prelude::*;
use crate::Vec3;

pub fn rand_vec<R: Rng>(rng: &mut R) -> Vec3 {
    Vec3::new(rng.gen(), rng.gen(), rng.gen())
}

pub fn rand_vec_range<R: Rng>(rng: &mut R, min: f64, max: f64) -> Vec3 {
    Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
}

pub fn rand_in_unit_sphere<R: Rng>(rng: &mut R) -> Vec3 {
    loop {
        let p = rand_vec_range(rng, -1.0, 1.0);
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

pub fn rand_unit_vec<R: Rng>(rng: &mut R) -> Unit<Vec3> {
    Unit::new_normalize(rand_in_unit_sphere(rng))
}

pub fn rand_in_hemisphere<R: Rng>(rng: &mut R, normal: &Vec3) -> Vec3 {
    let in_unit_sphere = rand_in_unit_sphere(rng);
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn rand_in_unit_disk<R: Rng>(rng: &mut R) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

pub fn near_zero(v: &Vec3) -> bool {
    let s = 1e-8;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}
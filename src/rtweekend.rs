use rand::distributions::{Distribution, Uniform};

pub const INFINITY: f64 = std::f64::INFINITY;

pub const PI: f64 = 3.1415926535897932385;

#[allow(dead_code)]
pub fn degrees_to_radians(degress: f64) -> f64 {
    degress * PI / 180.0
}

#[allow(dead_code)]
pub fn random_f64() -> f64 {
    random_range_f64(0.0, 1.0)
}

#[allow(dead_code)]
pub fn random_range_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let dis = Uniform::from(min..max);

    dis.sample(&mut rng)
}

#[allow(dead_code)]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }

    if x > max {
        return max;
    }

    x
}

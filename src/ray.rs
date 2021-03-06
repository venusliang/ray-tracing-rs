use super::vec3::*;

#[derive(Clone, Copy, Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(self) -> Point3 {
        self.orig
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn at(self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

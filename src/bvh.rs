use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::ray::*;
use crate::rtweekend::*;

#[derive(Clone)]
pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub abox: Aabb,
}

impl BvhNode {
    #[allow(dead_code)]
    pub fn bvh_node(
        src_objects: Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) {
        let objects = src_objects.clone();

        let axis = random_int(0, 2);

        // let comparator = if axis
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.abox.hit(&r, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self.right.hit(r, t_min, t_max, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.abox;
        true
    }
}

// pub fn box_compare (a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
//     // let mut box_a =
// }

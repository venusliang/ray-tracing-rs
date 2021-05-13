mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use camera::*;
use color::*;
use hittable::*;
use hittable_list::*;
use material::*;
use ray::*;
use rtweekend::*;
use sphere::*;
use std::sync::Arc;
use vec3::*;

fn ray_color<T: Hittable>(r: Ray, world: &T, depth: i64) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    let mut rec = HitRecord::default();

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();

        if rec
            .mat_ptr
            .scatter(&r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(scattered, world, depth - 1);
        }

        return Color::default();
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;
    const SAMPLES_PER_PIXEL: i64 = 100;
    const MAX_DEPTH: i64 = 50;

    // world
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // camera
    let cam = Camera::new();

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut j = IMAGE_HEIGHT - 1;
    while j >= 0 {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_f64()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_f64()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH)
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }

        j -= 1
    }
}

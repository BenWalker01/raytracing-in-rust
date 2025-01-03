use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
mod colour;
mod ray;
mod vec3;
use colour::{write_color, Colour};
use ray::Ray;
use vec3::Point3;
use vec3::Vec3;

fn ray_colour(r: &Ray) -> Colour {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let nrm = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return Colour::new(nrm.x + 1.0, nrm.y + 1.0, nrm.z + 1.0) * 0.5;
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Colour::new(
        (1.0 - t) * 1.0 + t * 0.5,
        (1.0 - t) * 1.0 + t * 0.7,
        (1.0 - t) * 1.0 + t * 1.0,
    )
}

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *r.origin() - center;
    let a = r.direction().dot(&r.direction());
    let b = 2.0 * oc.dot(&r.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn main() -> std::io::Result<()> {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, viewport_height, 0.0);
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(file);
    write!(writer, "P3\n{} {}\n255\n", image_width, image_height)?;

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_colour(&r);
            write_color(&mut writer, &pixel_color)?;
        }
    }
    println!("\rDone.                      \n");
    Ok(())
}

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
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    Colour::new(
        (1.0 - a) * 1.0 + a * 0.5,
        (1.0 - a) * 1.0 + a * 0.7,
        (1.0 - a) * 1.0 + a * 1.0,
    )
}

fn main() -> std::io::Result<()> {
    // Image

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 256;

    let image_height: i32 = image_width / aspect_ratio as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height = 2.0;
    let viewport_width: i32 = (viewport_height * (image_width as f64) / image_height as f64) as i32;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width as f64, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height as f64, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(file);
    write!(writer, "P3\n{} {}\n255\n", image_width, image_height)?;

    for j in 0..image_height {
        print!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_colour = ray_colour(&r);

            write_color(&mut writer, &pixel_colour)?;
        }
    }
    println!("\rDone.                      \n");
    Ok(())
}

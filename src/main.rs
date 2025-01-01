use std::fs::File;
use std::io::prelude::*;
mod vec3;
use vec3::Vec3;

fn main() -> std::io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let mut file = File::create("image.ppm")?;
    write!(file, "P3\n{} {}\n255\n", image_width, image_height)?;

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            write!(file, "{} {} {}\n", ir, ig, ib)?;
        }
    }
    println!("\rDone.                      \n");
    Ok(())
}

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
mod colour;
mod vec3;
use colour::{write_color, Colour};
use vec3::Vec3;

fn main() -> std::io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(file);
    write!(writer, "P3\n{} {}\n255\n", image_width, image_height)?;

    for j in (0..image_height) {
        print!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_color = Colour::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            write_color(&mut writer, &pixel_color)?;
        }
    }
    println!("\rDone.                      \n");
    Ok(())
}

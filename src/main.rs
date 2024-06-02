use simple_logger::SimpleLogger;
use std::io::{self, Write};

fn main() {
    SimpleLogger::new().init().unwrap();

    let image_width: i16 = 256;
    let image_height: i16 = 256;

    println!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let r = i as f32 / (image_width as f32 - 1.0);
            let g = j as f32 / (image_height as f32 - 1.0);
            let b: f32 = 0.0;

            let ir = (255.999 * r).trunc() as i16;
            let ig = (255.999 * g).trunc() as i16;
            let ib = (255.999 * b).trunc() as i16;

            println!("{ir} {ig} {ib}\n")
        }
    }

    eprint!("\rDone.                 \n");
    io::stderr().flush().unwrap();
}

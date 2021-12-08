extern crate openexr;

use half::f16;
use openexr::prelude::*;

#[derive(Default)]
struct RGBABuffer {
    data: Vec<Rgba>,
    width: i32,
    height: i32,
}

impl RGBABuffer {
    fn new(width: i32, height: i32) -> RGBABuffer {
        let capacity: usize = (width * height) as usize;
        let mut result = RGBABuffer {
            width,
            height,
            data: Vec::with_capacity(capacity),
        };
        result.data.resize(capacity, Rgba::default());

        result
    }
}

fn main() {
    let mut res = 1;
    while res < 256 {
        let file_name = format!("julia_{}k.exr", res);
        println!("Generating {}", file_name);
        println!(" - allocate buffer");
        let mut buffer = RGBABuffer::new(res * 1024, res * 1024);
        println!(" - generate fractal");
        generate_julia(&mut buffer, -0.8, 0.156);
        println!(" - write image");
        write_openexr(&file_name, buffer);
        res *= 2;
    }
}

fn generate_julia(buffer: &mut RGBABuffer, cx: f64, cy: f64) {
    let r = 2.0;
    let r2 = r * r;
    let max_iteration = 256;

    let mut index = 0;
    for y in 0..buffer.height {
        for x in 0..buffer.width {
            let mut zx = x as f64 / buffer.width as f64 - 0.5;
            let mut zy = y as f64 / buffer.height as f64 - 0.5;

            let mut iteration = 0;
            while zx * zx + zy * zy < r2 && iteration < max_iteration {
                let xtemp = zx * zx - zy * zy;
                zy = 2.0 * zx * zy + cy;
                zx = xtemp + cx;
                iteration += 1;
            }

            if iteration == max_iteration {
                index += 1;
                continue;
            }

            let value = f16::from_f32(iteration as f32 / max_iteration as f32);
            buffer.data[index].r = value;
            buffer.data[index].g = value;
            buffer.data[index].b = value;
            buffer.data[index].a = f16::from_f32(1.0);
            index += 1;
        }
        print!("{}/{}\r", y, buffer.height);
    }
}

fn write_openexr(file_name: &str, buffer: RGBABuffer) {
    // Create a file to write to.  The `Header` determines the properties of the
    // file, like resolution and what channels it has.
    let header = Header::from_dimensions(buffer.width, buffer.height);
    let mut file = RgbaOutputFile::new(file_name, &header, RgbaChannels::WriteRgba, 1).unwrap();
    file.set_frame_buffer(&buffer.data, 1, buffer.width as usize)
        .unwrap();
    unsafe {
        file.write_pixels(buffer.height).unwrap();
    }
}

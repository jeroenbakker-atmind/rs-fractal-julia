extern crate openexr;
extern crate rs_fractal_julia;

use openexr::prelude::*;

use rs_fractal_julia::buffer::BufferTrait;
use rs_fractal_julia::buffer::RGBABuffer;
use rs_fractal_julia::julia::CPUBackend;
use rs_fractal_julia::julia::Julia;
use rs_fractal_julia::julia::JuliaRow;

fn main() {
    let julia = Julia {
        r: 2.0,
        cx: -0.8,
        cy: 0.156,
        max_iteration: 256,
    };

    generate_openexrs(&julia, 1, 9);
}

fn generate_openexrs(julia: &Julia, from: u32, to: u32) {
    for res in from..to {
        generate_openexr_per_row(julia, res);
    }
}

fn generate_openexr_per_row(julia: &Julia, resolution: u32) {
    let file_name = format!("julia_{}k_row.exr", resolution);
    println!("Generating {}", file_name);
    println!(" - allocate buffer");
    let mut buffer = RGBABuffer::<Rgba>::new(resolution * 1024, resolution * 1024);
    println!(" - generate fractal");

    let res = resolution as usize * 1024;

    let r2 = julia.r * julia.r;

    let height = buffer.get_height();
    let backend = CPUBackend::<f32>::default();
    let mut row_buffer = Vec::with_capacity(res);

    let header = Header::from_dimensions(res as i32, res as i32);
    let mut file = RgbaOutputFile::new(file_name, &header, RgbaChannels::WriteRgba, 1).unwrap();
    file.set_frame_buffer(&buffer.data, 1, 0).unwrap();

    for y in 0..height {
        row_buffer.clear();
        backend.julia_row(julia, &mut row_buffer, res, res, y, r2);
        backend.store(julia, &mut buffer, 0, &row_buffer);

        unsafe {
            file.write_pixels(1).unwrap();
        }

        print!("{}/{}\r", y, height);
    }
}

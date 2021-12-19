#![feature(asm)]
#![feature(test)]
extern crate openexr;
extern crate png;
extern crate test;

use openexr::prelude::*;

#[cfg(test)]
mod benchmark;
mod buffer;
mod fixed_point;
mod julia;

use buffer::BufferTrait;
use buffer::RGBABuffer;
use julia::Julia;

fn main() {
    let julia = Julia {
        r: 2.0,
        cx: -0.8,
        cy: 0.156,
        max_iteration: 256,
    };

    generate_pngs(&julia, 1, 2);
    //generate_openexrs(&julia, 32, 33);
}

fn generate_pngs(julia: &Julia, from: u32, to: u32) {
    for res in from..to {
        generate_png(julia, res);
    }
}

fn generate_png(julia: &Julia, resolution: u32) {
    let file_name = format!("julia_{}k.png", resolution);
    println!("Generating {}", file_name);
    println!(" - allocate buffer");
    let mut buffer = RGBABuffer::<u8>::new(resolution * 1024, resolution * 1024);
    println!(" - generate fractal");
    julia.generate::<AsmXmm>(&mut buffer);
    println!(" - write image");
    write_png(&file_name, buffer);
}

fn generate_openexrs(julia: &Julia, from: u32, to: u32) {
    for res in from..to {
        generate_openexr_per_row(julia, res);
    }
}

fn generate_openexr(julia: &Julia, resolution: u32) {
    let file_name = format!("julia_{}k.exr", resolution);
    println!("Generating {}", file_name);
    println!(" - allocate buffer");
    let mut buffer = RGBABuffer::<Rgba>::new(resolution * 1024, resolution * 1024);
    println!(" - generate fractal");
    julia.generate::<CPUBackend<f32>>(&mut buffer);
    println!(" - write image");
    write_openexr(&file_name, buffer);
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

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::julia::CPUBackend;
use crate::julia::JuliaRow;
use crate::julia::AsmXmm;

fn write_png(file_name: &str, buffer: RGBABuffer<u8>) {
    let path = Path::new(file_name);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, buffer.width, buffer.height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_trns(vec![0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8]);
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&buffer.data).unwrap(); // Savea
}

fn write_openexr(file_name: &str, buffer: RGBABuffer<Rgba>) {
    // Create a file to write to.  The `Header` determines the properties of the
    // file, like resolution and what channels it has.
    let header = Header::from_dimensions(buffer.width as i32, buffer.height as i32);
    let mut file = RgbaOutputFile::new(file_name, &header, RgbaChannels::WriteRgba, 1).unwrap();
    file.set_frame_buffer(&buffer.data, 1, buffer.width as usize)
        .unwrap();
    unsafe {
        file.write_pixels(buffer.height as i32).unwrap();
    }
}

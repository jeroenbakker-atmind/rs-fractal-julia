extern crate openexr;
extern crate png;
use half::f16;
use openexr::prelude::*;

#[derive(Default)]
struct RGBABuffer<T> {
    data: Vec<T>,
    width: u32,
    height: u32,
}

pub trait BufferTrait {
    fn new(width: u32, height: u32) -> Self where Self:Sized;
fn    get_width(&self) -> u32;
fn    get_height(&self) -> u32;
fn    store_pixel(&mut self, offset:usize, value: f32); 
}

impl BufferTrait for RGBABuffer<u8> {
    fn new(width: u32, height: u32) -> RGBABuffer<u8> {
        let capacity: usize = (width * height * 4) as usize;
        let mut result = RGBABuffer {
            width,
            height,
            data: Vec::with_capacity(capacity),
        };
        result.data.resize(capacity, 0);

        result
    }

    fn get_width(&self) -> u32 {
        return self.width;
    }
    fn get_height(&self) -> u32 {
        return self.height;
    }
    fn store_pixel(&mut self, offset: usize, value: f32) {
        let value_u8 = (value * 255.0 ) as u8;
        let data_offset = offset * 4;
        self.data[data_offset] = value_u8;
        self.data[data_offset+1] = value_u8;
        self.data[data_offset+2] = value_u8;
        self.data[data_offset+3] = 255;
    }
}

impl BufferTrait for RGBABuffer<Rgba> {
    fn new(width: u32, height: u32) -> RGBABuffer<Rgba> {
        let capacity: usize = (width * height) as usize;
        let mut result = RGBABuffer {
            width,
            height,
            data: Vec::with_capacity(capacity),
        };
        result.data.resize(capacity, Rgba::default());

        result
    }

    fn get_width(&self) -> u32 {
        return self.width;
    }
    fn get_height(&self) -> u32 {
        return self.height;
    }
    fn store_pixel(&mut self, offset: usize, value: f32) {
        let value_half = f16::from_f32(value);
        let rgba = Rgba {
            r: value_half,
            g: value_half,
            b: value_half,
            a: f16::from_f32(1.0),
        };
        self.data[offset] = rgba;
    }
}

fn main() {
    let mut res = 1;
    while res <= 32 {
        let file_name = format!("julia_{}k.png", res);
        println!("Generating {}", file_name);
        println!(" - allocate buffer");
        let mut buffer = RGBABuffer::<u8>::new(res * 1024, res * 1024);
        println!(" - generate fractal");
        generate_julia(&mut buffer, -0.8, 0.156);
        println!(" - write image");
        write_png(&file_name, buffer);
        res += 1;
    }
    res = 1;
    while res <= 32 {
        let file_name = format!("julia_{}k.exr", res);
        println!("Generating {}", file_name);
        println!(" - allocate buffer");
        let mut buffer = RGBABuffer::<Rgba>::new(res * 1024, res * 1024);
        println!(" - generate fractal");
        generate_julia(&mut buffer, -0.8, 0.156);
        println!(" - write image");
        write_openexr(&file_name, buffer);
        res += 1;
    }
}

fn generate_julia(buffer: &mut dyn BufferTrait, cx: f64, cy: f64) {
    let r = 2.0;
    let r2 = r * r;
    let max_iteration = 256;

    let width = buffer.get_width();
    let height = buffer.get_height();

    let mut offset = 0;
    for y in 0..height {
        for x in 0..width {
            let mut zx = x as f64 / width as f64 - 0.5;
            let mut zy = y as f64 / height as f64 - 0.5;

            let mut iteration = 0;
            while zx * zx + zy * zy < r2 && iteration < max_iteration {
                let xtemp = zx * zx - zy * zy;
                zy = 2.0 * zx * zy + cy;
                zx = xtemp + cx;
                iteration += 1;
            }

            if iteration == max_iteration {
                offset += 1;
                continue;
            }

            let value = iteration as f32 / max_iteration as f32;
            buffer.store_pixel(offset, value);
            offset += 1;
        }
        print!("{}/{}\r", y, height);
    }
}

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
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


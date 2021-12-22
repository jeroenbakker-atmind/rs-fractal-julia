use half::f16;
use openexr::prelude::Rgba;

pub struct RGBABuffer<T> {
    pub data: Vec<T>,
    pub width: u32,
    pub height: u32,
}

pub trait BufferTrait {
    fn new(width: u32, height: u32) -> Self
    where
        Self: Sized;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn store_pixel(&mut self, offset: usize, value: f32);
    fn clear_pixel(&mut self, offset: usize);
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
        let value_u8 = (value * 255.0) as u8;
        let data_offset = offset * 4;
        self.data[data_offset] = value_u8;
        self.data[data_offset + 1] = value_u8;
        self.data[data_offset + 2] = value_u8;
        self.data[data_offset + 3] = 255;
    }

    fn clear_pixel(&mut self, offset: usize) {
        let data_offset = offset * 4;
        self.data[data_offset] = 0;
        self.data[data_offset + 1] = 0;
        self.data[data_offset + 2] = 0;
        self.data[data_offset + 3] = 0;
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
    fn clear_pixel(&mut self, offset: usize) {
        let value_half = f16::from_f32(0.0);
        let rgba = Rgba {
            r: value_half,
            g: value_half,
            b: value_half,
            a: value_half,
        };
        self.data[offset] = rgba;
    }
}

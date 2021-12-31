mod asmxmm_backend;
mod cpu_backend;

pub use asmxmm_backend::*;
pub use cpu_backend::*;

use crate::buffer::BufferTrait;

pub struct Julia {
    pub r: f32,
    pub cx: f32,
    pub cy: f32,
    pub max_iteration: usize,
}

impl Julia {
    pub fn generate<T>(&self, buffer: &mut dyn BufferTrait)
    where
        T: JuliaRow,
    {
        let r2 = self.r * self.r;

        let height = buffer.get_height();
        let backend = T::default();

        for y in 0..height {
            backend.julia_row_and_store(self, buffer, y, r2);
        }
    }
}

pub trait JuliaRow: Default {
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<u32>,
        width: usize,
        height: usize,
        row: u32,
        r2: f32,
    );

    fn julia_row_and_store(&self, julia: &Julia, buffer: &mut dyn BufferTrait, row: u32, r2: f32) {
        let width = buffer.get_width() as usize;
        let height = buffer.get_height() as usize;
        let mut row_buffer = Vec::with_capacity(width);
        self.julia_row(julia, &mut row_buffer, width, height, row, r2);
        assert_eq!(row_buffer.capacity(), width);
        assert_eq!(row_buffer.len(), width);
        self.store(julia, buffer, row, &row_buffer);
    }

    fn store(&self, julia: &Julia, buffer: &mut dyn BufferTrait, row: u32, row_buffer: &Vec<u32>) {
        let width = buffer.get_width() as usize;
        let mut offset = width * row as usize;
        for sample_offset in 0..width {
            self.store_pixel(
                row_buffer[sample_offset],
                julia.max_iteration as u32,
                buffer,
                offset,
            );
            offset += 1;
        }
    }

    fn store_pixel(
        &self,
        iteration: u32,
        max_iteration: u32,
        buffer: &mut dyn BufferTrait,
        offset: usize,
    ) {
        if iteration == max_iteration {
            buffer.clear_pixel(offset);
            return;
        }
        let value = iteration as f32 / max_iteration as f32;
        buffer.store_pixel(offset, value);
    }
}

impl JuliaRow for f32 {
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<u32>,
        width: usize,
        height: usize,
        row: u32,
        r2: f32,
    ) {
        let max_iteration = julia.max_iteration as u32;

        let t_width = width as f32;
        let t_half_width = t_width * 0.5;

        let rel_y = (row as f32 - height as f32 * 0.5) / height as f32;

        for x in 0..width as usize {
            let rel_x = (x as f32) - t_half_width;
            let mut zx = rel_x / t_width;
            let mut zy = rel_y;

            let mut iteration = 0;
            while zx * zx + zy * zy < r2 && iteration < max_iteration {
                let xtemp = zx * zx - zy * zy;
                zy = 2.0 * zx * zy + julia.cy;
                zx = xtemp + julia.cx;
                iteration += 1;
            }

            row_buffer.push(iteration);
        }
    }
}

impl JuliaRow for f64 {
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<u32>,
        width: usize,
        height: usize,
        row: u32,
        r2: f32,
    ) {
        let r2 = r2 as f64;
        let max_iteration = julia.max_iteration as u32;

        let t_width = width as f64;
        let t_half_width = t_width * 0.5;

        let rel_y = (row as f64 - height as f64 * 0.5) / height as f64;

        for x in 0..width as usize {
            let rel_x = (x as f64) - t_half_width;
            let mut zx = rel_x / t_width;
            let mut zy = rel_y;

            let mut iteration = 0;
            while zx * zx + zy * zy < r2 && iteration < max_iteration {
                let xtemp = zx * zx - zy * zy;
                zy = 2.0 * zx * zy + julia.cy as f64;
                zx = xtemp + julia.cx as f64;
                iteration += 1;
            }

            row_buffer.push(iteration);
        }
    }
}

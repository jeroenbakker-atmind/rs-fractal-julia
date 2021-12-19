use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

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
            print!("{}/{}\r", y, height);
        }
    }
}

pub trait JuliaRow: Default {
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<usize>,
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
        self.store(julia, buffer, row, &row_buffer);
    }

    fn store(
        &self,
        julia: &Julia,
        buffer: &mut dyn BufferTrait,
        row: u32,
        row_buffer: &Vec<usize>,
    ) {
        let width = buffer.get_width() as usize;
        let mut offset = width * row as usize;
        for sample_offset in 0..width {
            self.store_pixel(
                row_buffer[sample_offset],
                julia.max_iteration,
                buffer,
                offset,
            );
            offset += 1;
        }
    }

    fn store_pixel(
        &self,
        iteration: usize,
        max_iteration: usize,
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

pub struct CPUBackend<T> {
    phantom: PhantomData<T>,
}

impl<T> Default for CPUBackend<T> {
    fn default() -> CPUBackend<T> {
        CPUBackend::<T> {
            phantom: PhantomData::<T>::default(),
        }
    }
}

impl<T> JuliaRow for CPUBackend<T>
where
    T: Copy
        + From<f32>
        + Div<T, Output = T>
        + Mul<T, Output = T>
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + PartialOrd<T>,
{
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<usize>,
        width: usize,
        height: usize,
        row: u32,
        r2: f32,
    ) {
        let r2 = T::from(r2);
        let max_iteration = julia.max_iteration;

        let two = T::from(2.0);
        let cx = T::from(julia.cx);
        let cy = T::from(julia.cy);
        let t_width = T::from(width as f32);
        let t_half_width = T::from(width as f32 * 0.5);

        let rel_y = T::from((row as f32 - height as f32 * 0.5) / height as f32);

        for x in 0..width as usize {
            let rel_x = T::from(x as f32) - t_half_width;
            let mut zx = rel_x / t_width;
            let mut zy = rel_y;

            let mut iteration = 0;
            while zx * zx + zy * zy < r2 && iteration < max_iteration {
                let xtemp = zx * zx - zy * zy;
                zy = two * zx * zy + cy;
                zx = xtemp + cx;
                iteration += 1;
            }
            row_buffer.push(iteration);
        }
    }
}

impl JuliaRow for f32 {
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<usize>,
        width: usize,
        height: usize,
        row: u32,
        r2: f32,
    ) {
        let max_iteration = julia.max_iteration;

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

#[derive(Default)]
pub struct AsmXmm;

impl JuliaRow for AsmXmm {
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<usize>,
        width: usize,
        height: usize,
        row: u32,
        r2: f32,
    ) {
        let max_iteration = julia.max_iteration;

        let t_width = width as f32;
        let t_half_width = t_width * 0.5;

        let rel_y = (row as f32 - height as f32 * 0.5) / height as f32;

        for x in 0..width as usize {
            let rel_x = (x as f32) - t_half_width;
            let zx = rel_x / t_width;
            let zy = rel_y;

            let mut iteration = 0;
            /*
            while zx * zx + zy * zy < r2 && iteration < max_iteration {
                let xtemp = zx * zx - zy * zy;
                zy = 2.0 * zx * zy + julia.cy;
                zx = xtemp + julia.cx;
                iteration += 1;
            }*/

            unsafe {
                //xmm0 = zx
                //xmm1 = zy
                //xmm2 = r2
                //xmm3 = cy
                //xmm4 = cx
                //xmm5 = zx*zx
                //xmm6 = zy*zy
                //xmm7 = zx*zx+ zy*zy
                //xmm8 = zx*zx- zy*zy
                //eax = iteration Out only.
                //edx = max_iteration
                asm!(
                    "xor eax, eax",
                "2:",
                    "vmulss xmm5, xmm0, xmm0",
                    "vmulss xmm6, xmm1, xmm1",
                    "vaddss xmm7, xmm5, xmm6",
                    "comiss xmm7, xmm2",
                    "jnb 3f",
                    "cmp eax, edx",
                    "jnl 3f",
                    // xtemp
                    "vsubss xmm8, xmm5, xmm6",
                    // zy
                    "addss xmm1, xmm1",
                    "mulss xmm1, xmm0",
                    "addss xmm1, xmm3",
                    // zx
                    "vaddss xmm0, xmm8, xmm4",

                    "inc eax",
                    "jmp 2b",
                "3:",

                    in("xmm0") zx,
                    in("xmm1") zy,
                    in("xmm2") r2,
                    in("xmm3") julia.cy,
                    in("xmm4") julia.cx,
                    in("edx") max_iteration,
                    out("eax") iteration,
                );
            }

            row_buffer.push(iteration);
        }
    }
}

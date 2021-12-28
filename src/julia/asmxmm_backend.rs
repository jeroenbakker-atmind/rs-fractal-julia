use std::marker::PhantomData;

use crate::{julia_sample_xmm_f32_scalar, julia_sample_xmm_f64_scalar};

use super::{Julia, JuliaRow};

#[derive(Default)]
pub struct AsmX86<T> {
    phantom_data: PhantomData<T>,
}

#[repr(C)]
pub struct AsmX86Input<T> {
    // 0
    pub zy: T,
    // 4
    pub r2: T,
    // 8
    pub cx: T,
    // 12
    pub cy: T,
    // 16
    pub zx_min: T,
    // 20
    pub zx_max: T,
    // 24
    pub width: usize,
    // 32
    pub max_iteration: usize,
}

impl JuliaRow for AsmX86<f32> {
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<usize>,
        width: usize,
        height: usize,
        row: u32,
        r2: f32,
    ) {
        row_buffer.reserve_exact(width);
        unsafe {
            row_buffer.set_len(width);
        }
        let buffer = row_buffer.as_mut_ptr();
        let factor = row as f32 / height as f32;
        let min = -0.5;
        let max = -min;
        let zy = factor * max + (1.0 - factor) * min;
        unsafe {
            let parameters = AsmX86Input::<f32> {
                zy: zy,
                r2: r2,
                cx: julia.cx,
                cy: julia.cy,
                max_iteration: julia.max_iteration,
                zx_min: min,
                zx_max: max,
                width: width,
            };
            julia_sample_xmm_f32_scalar(buffer, &parameters);
        }
    }
}

impl JuliaRow for AsmX86<f64> {
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<usize>,
        width: usize,
        height: usize,
        row: u32,
        r2: f32,
    ) {
        row_buffer.reserve_exact(width);
        unsafe {
            row_buffer.set_len(width);
        }
        let buffer = row_buffer.as_mut_ptr();
        let factor = row as f64 / height as f64;
        let min = -0.5;
        let max = -min;
        let zy = factor * max + (1.0 - factor) * min;
        unsafe {
            let parameters = AsmX86Input::<f64> {
                zy: zy,
                r2: r2 as f64,
                cx: julia.cx as f64,
                cy: julia.cy as f64,
                max_iteration: julia.max_iteration,
                zx_min: min,
                zx_max: max,
                width: width,
            };
            julia_sample_xmm_f64_scalar(buffer, &parameters);
        }
    }
}

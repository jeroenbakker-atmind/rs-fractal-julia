use std::marker::PhantomData;

use crate::{
    julia_sample_xmm_f32_packed, julia_sample_xmm_f32_scalar, julia_sample_xmm_f64_packed,
    julia_sample_xmm_f64_scalar, julia_sample_ymm_f32_packed, julia_sample_ymm_f64_packed,
};

use super::{Julia, JuliaRow};

#[derive(Default)]
pub struct AsmXMMScalar<T> {
    phantom_data: PhantomData<T>,
}

#[derive(Default)]
pub struct AsmXMMPacked<T> {
    phantom_data: PhantomData<T>,
}

#[derive(Default)]
pub struct AsmYMMPacked<T> {
    phantom_data: PhantomData<T>,
}

#[repr(C)]
#[derive(Default)]
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
    pub max_iteration: u32,
    // 40
    pub scratch: [f32; 8],
}

impl JuliaRow for AsmXMMScalar<f32> {
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<u32>,
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
                max_iteration: julia.max_iteration as u32,
                zx_min: min,
                zx_max: max,
                width: width,
                ..AsmX86Input::<f32>::default()
            };
            julia_sample_xmm_f32_scalar(buffer, &parameters);
        }
    }
}

impl JuliaRow for AsmXMMPacked<f32> {
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<u32>,
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
                max_iteration: julia.max_iteration as u32,
                zx_min: min,
                zx_max: max,
                width: width,
                ..AsmX86Input::<f32>::default()
            };
            julia_sample_xmm_f32_packed(buffer, &parameters);
        }
    }
}

impl JuliaRow for AsmYMMPacked<f32> {
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<u32>,
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
                max_iteration: julia.max_iteration as u32,
                zx_min: min,
                zx_max: max,
                width: width,
                ..AsmX86Input::<f32>::default()
            };
            julia_sample_ymm_f32_packed(buffer, &parameters);
        }
    }
}

impl JuliaRow for AsmXMMScalar<f64> {
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<u32>,
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
                max_iteration: julia.max_iteration as u32,
                zx_min: min,
                zx_max: max,
                width: width,
                ..AsmX86Input::<f64>::default()
            };
            julia_sample_xmm_f64_scalar(buffer, &parameters);
        }
    }
}

impl JuliaRow for AsmXMMPacked<f64> {
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<u32>,
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
                max_iteration: julia.max_iteration as u32,
                zx_min: min,
                zx_max: max,
                width: width,
                ..AsmX86Input::<f64>::default()
            };
            julia_sample_xmm_f64_packed(buffer, &parameters);
        }
    }
}

impl JuliaRow for AsmYMMPacked<f64> {
    fn julia_row(
        &self,
        julia: &Julia,
        row_buffer: &mut Vec<u32>,
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
                max_iteration: julia.max_iteration as u32,
                zx_min: min,
                zx_max: max,
                width: width,
                ..AsmX86Input::<f64>::default()
            };
            julia_sample_ymm_f64_packed(buffer, &parameters);
        }
    }
}

#![feature(test)]
#![feature(vec_into_raw_parts)]

use julia::AsmX86Input;

#[cfg(test)]
extern crate test;

extern "C" {
    fn julia_sample_ymm_f32_packed(dest: *mut u32, parameters: &AsmX86Input<f32>);
    fn julia_sample_ymm_f64_packed(dest: *mut u32, parameters: &AsmX86Input<f64>);
    fn julia_sample_xmm_f32_packed(dest: *mut u32, parameters: &AsmX86Input<f32>);
    fn julia_sample_xmm_f32_scalar(dest: *mut u32, parameters: &AsmX86Input<f32>);
    fn julia_sample_xmm_f64_scalar(dest: *mut u32, parameters: &AsmX86Input<f64>);
    fn julia_sample_xmm_f64_packed(dest: *mut u32, parameters: &AsmX86Input<f64>);
}

#[cfg(test)]
mod benchmark;
pub mod buffer;
pub mod julia;

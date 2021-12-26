#![feature(asm)]
#![feature(test)]
#![feature(vec_into_raw_parts)]

use julia::AsmX86Input;

#[cfg(test)]
extern crate test;

extern "C" {
    fn julia_sample_xmm(dest: *mut usize, parameters: &AsmX86Input);
}

#[cfg(test)]
mod benchmark;
pub mod buffer;
pub mod julia;

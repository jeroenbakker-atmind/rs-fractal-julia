use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use crate::{buffer::BufferTrait, julia_sample_xmm};

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
        assert_eq!(row_buffer.capacity(), width);
        assert_eq!(row_buffer.len(), width);
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
pub struct AsmX86;

#[repr(C)]
pub struct AsmX86Input {
    pub cx: f32,
    pub cy: f32,
    pub iteration: usize,
}

impl JuliaRow for AsmX86 {
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
        let mut buffer = row_buffer.as_mut_ptr();

        let zy = (row as f32 - height as f32 * 0.5) / height as f32;
        let rx_add = 1.0 / width as f32;
        let mut zx = -0.5;

        for x in 0..width as usize {
            unsafe {
                let iteration;
                // Inputs:
                //xmm0 = zx
                //xmm1 = zy
                //xmm2 = r2
                //xmm3 = cy
                //xmm4 = cx
                //edi = address to store result (iteration)

                // Variables stored in registries.
                //xmm5 = zx*zx
                //xmm6 = zy*zy
                //edx = max_iteration

                // xmm7 scope is local so can be reused.
                //xmm7 = zx*zx+ zy*zy
                //xmm7 = zx*zx- zy*zy
                asm!(
                    "xor {9}, {9}",
                    "2:",
                    // update reg5xmm = (zx * zx)
                    "vmulss {5}, {0}, {0}",
                    // update reg6xmm = (zy * zy)
                    "vmulss {6}, {1}, {1}",
                    // update reg7xmm = (xmm5 + xmm6)
                    "vaddss {7}, {5}, {6}",
                    // Compare with reg2xmm (r2)
                    "comiss {7}, {2}",
                    "jnb 3f",
                    // Compare current iteration with max_iteration
                    "cmp {9}, {8}",
                    "jnl 3f",
                    // xmm7 = xtemp = (zx * zx - zy * zy)
                    "vsubss {7}, {5}, {6}",
                    // zy = 2 * zx * zy + cy
                    "addss {1}, {1}",
                    "mulss {1}, {0}",
                    "addss {1}, {3}",
                    // zx = xtemp + cx
                    "vaddss {0}, {7}, {4}",

                    // iteration += 1
                    "inc {9}",
                    "jmp 2b",
                    "3:",
                    //"mov qword ptr [{10}], {9}",
                    //"add {10}, 8",

                    inout(xmm_reg) zx => _,
                    inout(xmm_reg) zy => _,
                    in(xmm_reg) r2,
                    in(xmm_reg) julia.cy,
                    in(xmm_reg) julia.cx,
                    out(xmm_reg) _,
                    out(xmm_reg) _,
                    out(xmm_reg) _,
                    in(reg) julia.max_iteration,
                    out(reg) iteration,
                    // inout(reg) buffer,
                );
                let parameters = AsmX86Input {
                    cx: julia.cx,
                    cy: julia.cy,
                    iteration: iteration,
                };
                julia_sample_xmm(buffer.add(x), &parameters);
            }
            zx += rx_add;
        }
    }
}

#[test]
fn direct_buffer() {
    let mut vec = Vec::<usize>::new();
    vec.reserve_exact(100);

    unsafe {
        vec.set_len(100);
    }

    let ptr = vec.as_mut_ptr();
    for i in 0..100 {
        unsafe {
            *ptr.add(i) = i;
        }
    }

    for i in 0..100 {
        assert_eq!(i, vec[i]);
    }
}

#[test]
fn direct_buffer_asm() {
    let mut vec = Vec::<usize>::new();
    vec.reserve_exact(4096);

    unsafe {
        vec.set_len(4096);
    }

    let ptr = vec.as_mut_ptr();
    unsafe {
        asm!(
            "xor eax, eax",
            "mov edx, 4096",
        "2:",
            "mov [rdi], rax",
            "add rdi, 8",
            "inc eax",
            "cmp eax, edx",
            "jb 2b",
            in("rdi") ptr,
        );
    }

    for i in 0..4096 {
        unsafe {
            *ptr.add(i) = i;
        }
    }

    for i in 0..4096 {
        assert_eq!(i, vec[i]);
    }
}

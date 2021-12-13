use crate::buffer::BufferTrait;

pub struct Julia {
    pub r: f64,
    pub cx: f64,
    pub cy: f64,
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
            backend.julia_row(self, buffer, y, r2);
            print!("{}/{}\r", y, height);
        }
    }
}

pub trait JuliaRow: Default {
    fn julia_row(&self, julia: &Julia, buffer: &mut dyn BufferTrait, row: u32, r2: f64);
    fn store_pixel(
        &self,
        iteration: usize,
        max_iteration: usize,
        buffer: &mut dyn BufferTrait,
        offset: usize,
    ) {
        if iteration == max_iteration {
            return;
        }
        let value = iteration as f32 / max_iteration as f32;
        buffer.store_pixel(offset, value);
    }
}

impl JuliaRow for f64 {
    fn julia_row(&self, julia: &Julia, buffer: &mut dyn BufferTrait, row: u32, r2: f64) {
        let max_iteration = julia.max_iteration;
        let height = buffer.get_height();
        let width = buffer.get_width();

        let mut samples = vec![0_usize; width as usize];
        for x in 0..width as usize {
            let mut zx = x as f64 / width as f64 - 0.5;
            let mut zy = row as f64 / height as f64 - 0.5;

            let mut iteration = 0;
            let cx = julia.cx as f64;
            let cy = julia.cy as f64;
            while zx * zx + zy * zy < r2 && iteration < max_iteration {
                let xtemp = zx * zx - zy * zy;
                zy = 2.0 * zx * zy + cy;
                zx = xtemp + cx;
                iteration += 1;
            }

            samples[x] = iteration;
        }

        let mut offset = width as usize * row as usize;
        for sample_offset in 0..width as usize {
            self.store_pixel(samples[sample_offset], max_iteration, buffer, offset);
            offset += 1;
        }
    }
}

impl JuliaRow for f32 {
    fn julia_row(&self, julia: &Julia, buffer: &mut dyn BufferTrait, row: u32, r2: f64) {
        let r2 = r2 as f32;
        let max_iteration = julia.max_iteration;
        let height = buffer.get_height();
        let width = buffer.get_width();

        let mut samples = vec![0_usize; width as usize];
        for x in 0..width as usize {
            let mut zx = x as f32 / width as f32 - 0.5;
            let mut zy = row as f32 / height as f32 - 0.5;

            let mut iteration = 0;
            let cx = julia.cx as f32;
            let cy = julia.cy as f32;
            while zx * zx + zy * zy < r2 && iteration < max_iteration {
                let xtemp = zx * zx - zy * zy;
                zy = 2.0 * zx * zy + cy;
                zx = xtemp + cx;
                iteration += 1;
            }

            samples[x] = iteration;
        }

        let mut offset = width as usize * row as usize;
        for sample_offset in 0..width as usize {
            self.store_pixel(samples[sample_offset], max_iteration, buffer, offset);
            offset += 1;
        }
    }
}

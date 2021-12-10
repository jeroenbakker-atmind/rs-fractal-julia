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
        T: JuliaSample,
    {
        let r2 = self.r * self.r;

        let width = buffer.get_width();
        let height = buffer.get_height();
        let backend = T::default();

        let mut offset = 0;
        for y in 0..height {
            for x in 0..width {
                let zx = x as f64 / width as f64 - 0.5;
                let zy = y as f64 / height as f64 - 0.5;

                let iteration = backend.coord_stablization(self, zx, zy, r2);
                if iteration == self.max_iteration {
                    offset += 1;
                    continue;
                }

                let value = iteration as f32 / self.max_iteration as f32;
                buffer.store_pixel(offset, value);
                offset += 1;
            }
            print!("{}/{}\r", y, height);
        }
    }
}

pub trait JuliaSample: Default {
    fn coord_stablization(&self, julia: &Julia, x: f64, y: f64, r2: f64) -> usize;
}

impl JuliaSample for f64 {
    fn coord_stablization(&self, julia: &Julia, x: f64, y: f64, r2: f64) -> usize {
        let mut x = x;
        let mut y = y;
        let mut iteration = 0;
        let max_iteration = julia.max_iteration;
        let cx = julia.cx as f64;
        let cy = julia.cy as f64;
        while x * x + y * y < r2 && iteration < max_iteration {
            let xtemp = x * x - y * y;
            y = 2.0 * x * y + cy;
            x = xtemp + cx;
            iteration += 1;
        }
        iteration
    }
}

impl JuliaSample for f32 {
    fn coord_stablization(&self, julia: &Julia, x: f64, y: f64, r2: f64) -> usize {
        let mut x = x;
        let mut y = y;
        let mut iteration = 0;
        let max_iteration = julia.max_iteration;
        let cx = julia.cx as f64;
        let cy = julia.cy as f64;
        while x * x + y * y < r2 && iteration < max_iteration {
            let xtemp = x * x - y * y;
            y = 2.0 * x * y + cy;
            x = xtemp + cx;
            iteration += 1;
        }
        iteration
    }
}


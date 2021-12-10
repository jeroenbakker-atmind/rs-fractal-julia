use openexr::prelude::Rgba;
use test::Bencher;

use crate::{
    buffer::{BufferTrait, RGBABuffer},
    julia::Julia,
};

const BENCHMARK_RESOLUTION: u32 = 256;

#[bench]
fn bench_f64(bench: &mut Bencher) {
    let julia = Julia {
        cx: -0.8,
        cy: 0.156,
        r: 2.0,
        max_iteration: 256,
    };
    let mut buffer = RGBABuffer::<Rgba>::new(BENCHMARK_RESOLUTION, BENCHMARK_RESOLUTION);
    bench.iter(|| {
        julia.generate::<f64>(&mut buffer);
    })
}

#[bench]
fn bench_f32(bench: &mut Bencher) {
    let julia = Julia {
        cx: -0.8,
        cy: 0.156,
        r: 2.0,
        max_iteration: 256,
    };
    let mut buffer = RGBABuffer::<Rgba>::new(BENCHMARK_RESOLUTION, BENCHMARK_RESOLUTION);
    bench.iter(|| {
        julia.generate::<f32>(&mut buffer);
    })
}

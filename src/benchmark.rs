use openexr::prelude::Rgba;
use test::Bencher;

use crate::{
    buffer::{BufferTrait, RGBABuffer},
    fixed_point::FixedPoint,
    julia::{CPUBackend, Julia},
};

const BENCHMARK_RESOLUTION: u32 = 256;

#[bench]
fn bench_cpu_f64(bench: &mut Bencher) {
    let julia = Julia {
        cx: -0.8,
        cy: 0.156,
        r: 2.0,
        max_iteration: 256,
    };
    let mut buffer = RGBABuffer::<Rgba>::new(BENCHMARK_RESOLUTION, BENCHMARK_RESOLUTION);
    bench.iter(|| {
        julia.generate::<CPUBackend<f64>>(&mut buffer);
    })
}

#[bench]
fn bench_cpu_f32(bench: &mut Bencher) {
    let julia = Julia {
        cx: -0.8,
        cy: 0.156,
        r: 2.0,
        max_iteration: 256,
    };
    let mut buffer = RGBABuffer::<Rgba>::new(BENCHMARK_RESOLUTION, BENCHMARK_RESOLUTION);
    bench.iter(|| {
        julia.generate::<CPUBackend<f32>>(&mut buffer);
    })
}
#[bench]
fn bench_cpu_i64(bench: &mut Bencher) {
    let julia = Julia {
        cx: -0.8,
        cy: 0.156,
        r: 2.0,
        max_iteration: 256,
    };
    let mut buffer = RGBABuffer::<Rgba>::new(BENCHMARK_RESOLUTION, BENCHMARK_RESOLUTION);
    bench.iter(|| {
        julia.generate::<CPUBackend<FixedPoint<i64>>>(&mut buffer);
    })
}

#[bench]
fn bench_cpu_i32(bench: &mut Bencher) {
    let julia = Julia {
        cx: -0.8,
        cy: 0.156,
        r: 2.0,
        max_iteration: 256,
    };
    let mut buffer = RGBABuffer::<Rgba>::new(BENCHMARK_RESOLUTION, BENCHMARK_RESOLUTION);
    bench.iter(|| {
        julia.generate::<CPUBackend<FixedPoint<i32>>>(&mut buffer);
    })
}

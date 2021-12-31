use test::Bencher;

use crate::{
    buffer::{BufferTrait, RGBABuffer},
    julia::{AsmXMMPacked, AsmXMMScalar, CPUBackend, Julia},
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
    let mut buffer = RGBABuffer::<u8>::new(BENCHMARK_RESOLUTION, BENCHMARK_RESOLUTION);
    bench.iter(|| {
        julia.generate::<CPUBackend<f64>>(&mut buffer);
    })
}

#[bench]
fn bench_native_f32(bench: &mut Bencher) {
    let julia = Julia {
        cx: -0.8,
        cy: 0.156,
        r: 2.0,
        max_iteration: 256,
    };
    let mut buffer = RGBABuffer::<u8>::new(BENCHMARK_RESOLUTION, BENCHMARK_RESOLUTION);
    bench.iter(|| {
        julia.generate::<f32>(&mut buffer);
    })
}

#[bench]
fn bench_native_f64(bench: &mut Bencher) {
    let julia = Julia {
        cx: -0.8,
        cy: 0.156,
        r: 2.0,
        max_iteration: 256,
    };
    let mut buffer = RGBABuffer::<u8>::new(BENCHMARK_RESOLUTION, BENCHMARK_RESOLUTION);
    bench.iter(|| {
        julia.generate::<f64>(&mut buffer);
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
    let mut buffer = RGBABuffer::<u8>::new(BENCHMARK_RESOLUTION, BENCHMARK_RESOLUTION);
    bench.iter(|| {
        julia.generate::<CPUBackend<f32>>(&mut buffer);
    })
}

#[bench]
fn bench_asm_xmm_f32_scalar(bench: &mut Bencher) {
    let julia = Julia {
        cx: -0.8,
        cy: 0.156,
        r: 2.0,
        max_iteration: 256,
    };
    let mut buffer = RGBABuffer::<u8>::new(BENCHMARK_RESOLUTION, BENCHMARK_RESOLUTION);
    bench.iter(|| {
        julia.generate::<AsmXMMScalar<f32>>(&mut buffer);
    })
}

#[bench]
fn bench_asm_xmm_f32_packed(bench: &mut Bencher) {
    let julia = Julia {
        cx: -0.8,
        cy: 0.156,
        r: 2.0,
        max_iteration: 256,
    };
    let mut buffer = RGBABuffer::<u8>::new(BENCHMARK_RESOLUTION, BENCHMARK_RESOLUTION);
    bench.iter(|| {
        julia.generate::<AsmXMMPacked<f32>>(&mut buffer);
    })
}

#[bench]
fn bench_asm_xmm_f64_scalar(bench: &mut Bencher) {
    let julia = Julia {
        cx: -0.8,
        cy: 0.156,
        r: 2.0,
        max_iteration: 256,
    };
    let mut buffer = RGBABuffer::<u8>::new(BENCHMARK_RESOLUTION, BENCHMARK_RESOLUTION);
    bench.iter(|| {
        julia.generate::<AsmXMMScalar<f64>>(&mut buffer);
    })
}

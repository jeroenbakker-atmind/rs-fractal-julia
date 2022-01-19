Julia fractal generator for rust.

Fractal generation that (eventually) should support huge image sizes. Done as it is hard to find
free images with huge resolution.

The second goal of the project is an exercise at writing vectorization code (AVX1/AVX2) in assembly. Therefore the solution doesn't support parallelization across multiple CPU cores.
The code has room for improvements (performance wise) as the goal was to keep the code readable.

Rust is used to export the data to an image, compare performance and correctness.

Results running on an Intel(R) Core(TM) i7-8550U CPU

```
test benchmark::bench_asm_xmm_f32_packed ... bench:   7,745,991 ns/iter (+/- 658,788)
test benchmark::bench_asm_xmm_f32_scalar ... bench:  21,978,492 ns/iter (+/- 1,335,452)
test benchmark::bench_asm_xmm_f64_packed ... bench:  13,083,099 ns/iter (+/- 704,691)
test benchmark::bench_asm_xmm_f64_scalar ... bench:  21,945,079 ns/iter (+/- 1,216,690)
test benchmark::bench_asm_ymm_f32_packed ... bench:   4,826,656 ns/iter (+/- 186,970)
test benchmark::bench_cpu_f32            ... bench:  22,353,429 ns/iter (+/- 1,148,159)
test benchmark::bench_cpu_f64            ... bench:  26,983,873 ns/iter (+/- 1,321,683)
test benchmark::bench_native_f32         ... bench:  22,502,146 ns/iter (+/- 710,653)
test benchmark::bench_native_f64         ... bench:  26,797,124 ns/iter (+/- 982,912)
```

Remarkably double precision is slower in rust. This could be to bad vectorization. Still need to have a look at the generated assembly.

Modern CPU only can calculate doubles, and use bit sizzling to support floats. Hence the small difference when between the scalar implementations.

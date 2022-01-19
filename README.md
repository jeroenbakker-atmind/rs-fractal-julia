Julia fractal generator for rust.

Fractal generation that (eventually) should support huge image sizes. Done as it is hard to find
free images with huge resolution.

The second goal of the project is an exercise at writing vectorization code (AVX1/AVX2) in assembly. Therefore the solution doesn't support parallelization across multiple CPU cores.
The code has room for improvements (performance wise) as the goal was to keep the code readable.

Rust is used to export the data to an image, compare performance and correctness.

Results running on an Intel(R) Core(TM) i7-8550U CPU (Slowest on top).

```
test benchmark::bench_native_f64         ... bench:  26,907,733 ns/iter (+/- 1,039,052)
test benchmark::bench_cpu_f64            ... bench:  26,821,107 ns/iter (+/- 971,015)
test benchmark::bench_cpu_f32            ... bench:  22,525,174 ns/iter (+/- 961,333)
test benchmark::bench_native_f32         ... bench:  22,246,048 ns/iter (+/- 1,016,533)
test benchmark::bench_asm_xmm_f64_scalar ... bench:  21,793,094 ns/iter (+/- 929,025)
test benchmark::bench_asm_xmm_f32_scalar ... bench:  21,760,044 ns/iter (+/- 1,005,370)
test benchmark::bench_asm_xmm_f64_packed ... bench:  12,938,314 ns/iter (+/- 685,412)
test benchmark::bench_asm_ymm_f64_packed ... bench:   7,932,974 ns/iter (+/- 401,188)
test benchmark::bench_asm_xmm_f32_packed ... bench:   7,837,280 ns/iter (+/- 429,013)
test benchmark::bench_asm_ymm_f32_packed ... bench:   4,821,649 ns/iter (+/- 338,150)
```

Remarkably double precision is slower in rust. This could be to bad vectorization. Still need to have a look at the generated assembly.

Modern CPU only can calculate doubles, and use bit sizzling to support floats. Hence the small difference when between the scalar implementations.

Note: That this has been developed on a Linux OS and  hasn't been tested on other OS's.
Note: Will only compile and run on AVX2 X86 processors.

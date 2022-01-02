Julia fractal generator for rust.

Fractal generation that (eventually) should support huge image sizes. Done as it is hard to find
free images with huge resolution.

A sub-goal of the project is an exercise at writing vectorization code (AVX) in assembly. Therefore the solution doesn't support parallelization across multiple CPU cores.

Rust is used to export the data to an image, compare performance and correctness.

Results running on an Intel(R) Core(TM) i7-8550U CPU
```
test benchmark::bench_asm_xmm_f32_packed ... bench:   8,030,902 ns/iter (+/- 726,648)
test benchmark::bench_asm_xmm_f32_scalar ... bench:  22,494,697 ns/iter (+/- 2,721,733)
test benchmark::bench_asm_xmm_f64_scalar ... bench:  21,933,574 ns/iter (+/- 1,151,042)
test benchmark::bench_cpu_f32            ... bench:  22,567,612 ns/iter (+/- 1,095,805)
test benchmark::bench_cpu_f64            ... bench:  26,711,851 ns/iter (+/- 1,218,051)
test benchmark::bench_native_f32         ... bench:  22,272,041 ns/iter (+/- 746,111)
test benchmark::bench_native_f64         ... bench:  26,861,162 ns/iter (+/- 7,074,162)
```

Remarkably double precision is slower in rust. This could be to bad vectorization. Still need to have a look at the generated assembly.

Modern CPU only can calculate doubles, and use bit sizzling to support floats. Hence the small difference when between the scalar implementations.

When vectorizing the performance for doubles should be less as half of the data can be kept in the registers.
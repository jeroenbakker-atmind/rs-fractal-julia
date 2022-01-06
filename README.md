Julia fractal generator for rust.

Fractal generation that (eventually) should support huge image sizes. Done as it is hard to find
free images with huge resolution.

A sub-goal of the project is an exercise at writing vectorization code (AVX) in assembly. Therefore the solution doesn't support parallelization across multiple CPU cores.

Rust is used to export the data to an image, compare performance and correctness.

Results running on an Intel(R) Core(TM) i7-8550U CPU
```
test benchmark::bench_asm_xmm_f32_packed ... bench:   7,879,546 ns/iter (+/- 461,113)
test benchmark::bench_asm_xmm_f32_scalar ... bench:  22,110,853 ns/iter (+/- 1,130,467)
test benchmark::bench_asm_xmm_f64_packed ... bench:  12,915,632 ns/iter (+/- 802,999)
test benchmark::bench_asm_xmm_f64_scalar ... bench:  21,998,341 ns/iter (+/- 1,136,655)
test benchmark::bench_cpu_f32            ... bench:  22,412,409 ns/iter (+/- 1,269,286)
test benchmark::bench_cpu_f64            ... bench:  26,766,961 ns/iter (+/- 1,191,797)
test benchmark::bench_native_f32         ... bench:  22,627,857 ns/iter (+/- 1,945,357)
test benchmark::bench_native_f64         ... bench:  26,652,314 ns/iter (+/- 1,137,777)
```

Remarkably double precision is slower in rust. This could be to bad vectorization. Still need to have a look at the generated assembly.

Modern CPU only can calculate doubles, and use bit sizzling to support floats. Hence the small difference when between the scalar implementations.

When vectorizing the performance for doubles should be less as half of the data can be kept in the registers.
extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/julia-xmm-f32-packed.S");
    println!("cargo:rerun-if-changed=src/julia-xmm-f32-scalar.S");
    println!("cargo:rerun-if-changed=src/julia-xmm-f64-packed.S");
    println!("cargo:rerun-if-changed=src/julia-xmm-f64-scalar.S");
    println!("cargo:rerun-if-changed=src/julia-ymm-f32-packed.S");
    println!("cargo:rerun-if-changed=src/julia-ymm-f64-packed.S");
    cc::Build::new()
        .flag("-Wa,-mmnemonic=intel")
        .flag("-Wa,-msyntax=intel")
        .flag("-Wa,-mnaked-reg")
        .file("src/julia-xmm-f32-packed.S")
        .file("src/julia-xmm-f32-scalar.S")
        .file("src/julia-xmm-f64-packed.S")
        .file("src/julia-xmm-f64-scalar.S")
        .file("src/julia-ymm-f32-packed.S")
        .file("src/julia-ymm-f64-packed.S")
        .compile("juliaasm");
}

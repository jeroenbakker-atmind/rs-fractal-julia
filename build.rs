extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/julia-asm.S");
    cc::Build::new()
        .flag("-Wa,-mmnemonic=intel")
        .flag("-Wa,-msyntax=intel")
        .flag("-Wa,-mnaked-reg")
        .file("src/julia-asm.S")
        .compile("juliaasm");
}

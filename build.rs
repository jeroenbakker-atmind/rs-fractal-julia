extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/julia-asm.S");
    cc::Build::new().file("src/julia-asm.S").compile("juliaasm");
}

fn main() {
    println!("cargo:rerun-if-changed=target/debug/cr.o");
    println!("cargo:rustc-link-search=target/debug");
    // println!("cargo:rustc-link-lib=target/debug/cr");
    // println!("cargo:rerun-if-changed=../../target/debug");

    cc::Build::new()
        .object("../../target/debug/cr.o")
        .compile("cr");
}

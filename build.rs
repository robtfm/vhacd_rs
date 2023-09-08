fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/cxx_vhacd.cc")
        .flag_if_supported("-std=c++14")
        .compile("vhacd_rs");

    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=include");
}

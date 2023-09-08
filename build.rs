fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/cxx_vhacd.cc")
        .compile("vhacd_rs");


    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=include");    
}
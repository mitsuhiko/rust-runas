extern crate gcc;

use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "windows" {
        gcc::compile_library("librunas.a", &["runas-windows.c"]);
        println!("cargo:rustc-link-lib=runas");
        println!("cargo:rustc-link-lib=ole32");
    } else if target_os == "macos" {
        gcc::compile_library("librunas.a", &["runas-darwin.c"]);
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=runas");
    }
}

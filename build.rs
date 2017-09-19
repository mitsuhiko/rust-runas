extern crate gcc;

use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "windows" {
        gcc::Build::new()
            .file("runas-windows.c")
            .compile("runas");
        println!("cargo:rustc-link-lib=ole32");
    } else if target_os == "macos" {
        gcc::Build::new()
            .file("runas-darwin.c")
            .compile("runas");
        println!("cargo:rustc-link-lib=framework=Security");
    }
}

use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "windows" {
        cc::Build::new().file("runas-windows.c").compile("runas");
        println!("cargo:rustc-link-lib=ole32");
    }
}

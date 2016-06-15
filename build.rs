extern crate gcc;

fn main() {
    if cfg!(windows) {
        gcc::compile_library("librunas.a", &["runas-windows.c"]);
        println!("cargo:rustc-link-lib=runas");
        println!("cargo:rustc-link-lib=ole32");
    } else if cfg!(target_os = "macos") {
        gcc::compile_library("librunas.a", &["runas-darwin.c"]);
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=runas");
    }
}

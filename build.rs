extern crate gcc;

fn main() {
    if cfg!(target_os = "windows") {
        gcc::compile_library("librunas.a", &["runas-windows.c"]);
        println!("cargo:rustc-link-lib=runas");
        println!("cargo:rustc-link-lib=ole32");
    }
}

fn main() {
    let runtime_dir = std::env::var("DEP_BOXLITE_RUNTIME_DIR").unwrap();
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,{runtime_dir}");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,{runtime_dir}");
}

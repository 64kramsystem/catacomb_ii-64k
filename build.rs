#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    println!("cargo:rustc-flags=-l SDL2");
}

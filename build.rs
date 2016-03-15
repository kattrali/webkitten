use std::env;

fn main() {
    match env::var("CFLAGS") {
        Ok(cflags) => println!("cargo:rustc-flags={}", cflags),
        _ => {}
    }
}

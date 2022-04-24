use std::{env, error::Error, fs::File, io::Write, path::PathBuf};
use cc;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-search={}/lib/", env::current_dir().unwrap().to_str().unwrap());
    File::create(out_dir.join("script.ld"))?.write_all(include_bytes!("script.ld"))?;
    println!("cargo:rustc-link-lib=nosys");
    println!("cargo:rustc-link-lib=c_nano");

    // Assembly code
    cc::Build::new().file("init.s").compile("asm");
    println!("cargo:rerun-if-changed=init.s");

    Ok(())
}

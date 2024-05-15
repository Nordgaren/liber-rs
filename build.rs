use std::io::{Error, ErrorKind};

fn main() -> std::io::Result<()> {
    let path1 = std::path::PathBuf::from(r"C:\Users\Nordgaren\source\C++\libER\include"); // include path
    let path2 = std::path::PathBuf::from(r"C:\Users\Nordgaren\source\C++\libER\source"); // include path
    let mut b = autocxx_build::Builder::new("src/include.rs", &[&path1, &path2])
        .extra_clang_args(&["-std=c++20"])
        .build()
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e.to_string()))?;
    // This assumes all your C++ bindings are in src/include.rs
    b.flag_if_supported("-std=c++20").compile("liber-rs"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/main.rs");
    // Add instructions to link to any C++ libraries you need.
    Ok(())
}
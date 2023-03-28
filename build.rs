use miette::miette;
use std::path::PathBuf;

fn main() -> miette::Result<()> {
    println!("cargo:rerun-if-changed=cxx/include/gen.hxx");
    println!("cargo:rerun-if-changed=src/gen.rs");
    let mut build = autocxx_build::Builder::new(PathBuf::from_iter(["src", "gen.rs"]), [PathBuf::from_iter([
        "cxx", "include",
    ])])
    .extra_clang_args(&["-std=gnu++17"])
    .build()?;
    build
        .flag_if_supported("-std=gnu++17")
        .try_compile("autocxx-crash")
        .map_err(|e| miette!(e))?;
    Ok(())
}

use std::env;

fn main() {
    let crate_type = env::var("CARGO_CFG_CRATE_TYPE").unwrap_or(String::from("unknown"));

    match crate_type.as_str() {
        "cdylib" | "bin" => {
            println!("cargo:rustc-cfg=skip_pyo3");
        }
        _ => {
            println!("cargo:rustc-cfg=skip_pyo3");
        }
    }
}

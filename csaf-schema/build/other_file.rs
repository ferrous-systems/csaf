use std::path::PathBuf;

pub fn build_rs() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let base_path = PathBuf::from(out_dir).join("schema-build-dir");

    std::fs::create_dir_all(&base_path).unwrap();

    std::fs::write(
        base_path.join("build_out_file.txt"),
        format!("File written from module='{}' in file: {}", module_path!(), file!()),
    )
    .unwrap();

    // To execute the build script, run `cargo build -p csaf-schema`
    // Note: The created file will be located somewhere under `target/debug/build/csaf-schema-<some hash>/out/schema-build-dir/build_out_file.txt`
}

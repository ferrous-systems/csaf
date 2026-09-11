use std::{env, path::Path};

const GENERATED_FOLDER_NAME: &str = "generated";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join(GENERATED_FOLDER_NAME);

    println!("cargo:rerun-if-changed=../csaf"); // csaf submodule

    let dir = env!("CARGO_MANIFEST_DIR");
    let bundled_path = Path::new(dir).join(GENERATED_FOLDER_NAME);

    // TODO:
    // 1. Generate code using 'out_path' as initial target
    // 2. Apply optional cleanup to generated code
    // 3. copy resulting generated code to `bundled_path` to check into git
}

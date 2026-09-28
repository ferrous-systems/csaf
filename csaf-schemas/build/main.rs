use std::path::Path;

mod other_file;

const GENERATED_FOLDER_NAME: &str = "generated";

pub fn main() {
    println!("cargo:rerun-if-changed=schema-links/csaf2_1.json"); // csaf schema for v2.1

    let dir = env!("CARGO_MANIFEST_DIR");
    let _bundled_path = Path::new(dir).join(GENERATED_FOLDER_NAME);

    let _schema2_1 = std::fs::read_to_string("schema-links/csaf2_1.json").unwrap();

    // TODO:
    // 1. Generate code in path pointed to by environmental variable 'OUT_DIR'
    // 2. Apply optional cleanup to generated code
    // 3. Optional: Copy resulting generated code to `bundled_path` to check into git

    // use of function from linked module
    other_file::build_rs();
}

use std::env::current_dir;
use std::{env, fs};

fn main() {
    let goats_dir = current_dir().unwrap().join("src").join("goats");

    let goat_files = fs::read_dir(&goats_dir)
        .unwrap()
        .into_iter()
        .flat_map(|e| match e {
            Ok(f) => f.file_name().to_str().map(|s| s.to_string()),
            Err(_) => None,
        })
        .map(|e| goats_dir.join(e))
        .flat_map(|e| e.to_str().map(|e| e.to_string()))
        .map(|e| format!("include_bytes!(\"{e}\")"))
        .collect::<Vec<String>>();

    let file = format!(
        r#"
const GOATS:[&[u8]; {len}] = [
    {includes}
];"#,
        len = goat_files.len(),
        includes = goat_files.join(",\n")
    );

    fs::write(format!("{}/goats.rs", env::var("OUT_DIR").unwrap()), file).unwrap();
}

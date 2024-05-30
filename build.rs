use std::env::current_dir;
use std::{env, fs};

fn main() {
    let goats_dir = current_dir().unwrap().join("src").join("goats");

    let goat_files = fs::read_dir(&goats_dir)
        .unwrap()
        .flat_map(|e| match e {
            Ok(f) => f.file_name().to_str().map(|s| s.to_string()),
            Err(_) => None,
        })
        .map(|e| goats_dir.join(e))
        .flat_map(|e| {
            match (
                mime_guess::from_path(&e).first(),
                e.to_str().map(|e| e.to_string()),
            ) {
                (Some(mime), Some(path)) => Some((mime.to_string(), path)),
                _ => None,
            }
        })
        .map(|(mime, path)| {
            format!("Image {{ data: include_bytes!(\"{path}\"), mime: \"{mime}\" }}")
        })
        .collect::<Vec<String>>();

    let file = format!(
        r#"
const GOATS:[Image; {len}] = [
    {includes}
];"#,
        len = goat_files.len(),
        includes = goat_files.join(",\n")
    );

    fs::write(format!("{}/goats.rs", env::var("OUT_DIR").unwrap()), file).unwrap();
}

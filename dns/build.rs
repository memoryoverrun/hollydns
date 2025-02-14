use std::fs;
use std::path::Path;

fn main() {
    let dir = "src/rdatas";
    let output_file = "src/rdatas_generated.rs";

    if !Path::new(dir).exists() {
        eprintln!("dir {} not found", dir);
        return;
    }

    let mods: Vec<String> = fs::read_dir(dir)
        .expect("read dir failed")
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if path.extension()?.to_str()? == "rs" {
                let filename = path.file_stem()?.to_str()?.to_string();
                if filename != "mod" {
                    return Some(format!("pub mod {} {{ include!(\"rdatas/{}.rs\"); }}", filename, filename));
                }
            }
            None
        })
        .collect();

    let content = mods.join("\n");

    fs::write(output_file, content).expect("failed to write rdatas_generated.rs");

    println!("cargo:rerun-if-changed=src/rdatas");
}

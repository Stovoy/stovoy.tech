use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let dest_path = out_dir.join("file_map.rs");

    let workspace_root = locate_workspace_root();

    let mut entries = Vec::new();

    gather_files(&workspace_root, &mut entries);

    entries.sort();

    let mut file = File::create(&dest_path).expect("Failed to create file_map.rs");

    writeln!(file, "use std::collections::HashMap;").unwrap();
    writeln!(
        file,
        "pub fn build_file_map() -> HashMap<&'static str, &'static str> {{"
    )
    .unwrap();
    writeln!(file, "    let mut map = HashMap::new();").unwrap();

    for (rel_path, abs_path) in entries {
        writeln!(
            file,
            "    map.insert({:?}, include_str!({:?}));",
            rel_path, abs_path
        )
        .unwrap();
    }

    writeln!(file, "    map").unwrap();
    writeln!(file, "}}").unwrap();
}

fn locate_workspace_root() -> PathBuf {
    let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    while dir.parent().is_some() {
        if dir.join("Cargo.toml").exists()
            && dir.join("backend").exists()
            && (dir.join("frontend_rust").exists() || dir.join("frontend").exists())
        {
            return dir;
        }
        dir = dir.parent().unwrap().to_path_buf();
    }
    panic!("Workspace root not found");
}

fn gather_files(root: &Path, entries: &mut Vec<(String, String)>) {
    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_ignored(e.path()))
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.into_path();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if matches!(ext, "rs" | "toml" | "css" | "html") {
                let abs = path.to_string_lossy().to_string();
                let rel = path
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                entries.push((rel, abs));
            }
        }
    }
}

fn is_ignored(path: &Path) -> bool {
    let components: Vec<_> = path
        .components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect();
    components.iter().any(|comp| {
        comp.starts_with('.')
            || comp == "target"
            || comp == "dist"
            || comp == "node_modules"
            || comp == "git"
            || comp == "frontend_rust"
    })
}

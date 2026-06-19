use std::path::{Path, PathBuf};

include!("src/assets_config.rs");

fn find_asset_dir(root: &Path, filename: &str) -> PathBuf {
    let mut found = Vec::new();
    search_dirs(root, filename, &mut found);
    match found.len() {
        0 => panic!("no directory found in '{}' containing '{}'", root.display(), filename),
        1 => found.remove(0),
        n => panic!(
            "{} directories found containing '{}' under '{}': {:#?}",
            n,
            filename,
            root.display(),
            found
        ),
    }
}

fn search_dirs(dir: &Path, filename: &str, found: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut subdirs = Vec::new();
    let mut has_file = false;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            subdirs.push(path);
        } else if path.file_name().map(|n| n == filename).unwrap_or(false) {
            has_file = true;
        }
    }
    if has_file {
        found.push(dir.to_path_buf());
    }
    for subdir in subdirs {
        search_dirs(&subdir, filename, found);
    }
}

fn copy_assets_by_stem(src: &Path, dest_dir: &Path) {
    let stem = src
        .file_stem()
        .expect("asset path has no file stem")
        .to_string_lossy()
        .into_owned();
    let src_dir = src.parent().expect("asset path has no parent directory");
    let entries = std::fs::read_dir(src_dir)
        .unwrap_or_else(|e| panic!("failed to read directory {}: {}", src_dir.display(), e));
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && path.file_stem().map(|s| s == stem.as_str()).unwrap_or(false) {
            let filename = path.file_name().unwrap();
            let dest = dest_dir.join(filename);
            std::fs::copy(&path, &dest).unwrap_or_else(|e| {
                panic!("failed to copy {} to {}: {}", path.display(), dest.display(), e)
            });
        }
    }
}

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let manifest_path = Path::new(&manifest_dir);
    let assets_submodule = manifest_path.join("../aisle9_assets");
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .expect("OUT_DIR does not have enough ancestors")
        .to_path_buf();

    let assets_hjson_src = manifest_path.join("assets.hjson");
    println!("cargo:rerun-if-changed=assets.hjson");
    println!("cargo:rerun-if-changed=../aisle9_assets");

    let hjson_text = std::fs::read_to_string(&assets_hjson_src).expect("failed to read assets.hjson");
    let assets: Assets = deser_hjson::from_str(&hjson_text).expect("failed to parse assets.hjson");

    let assets_out = target_dir.join("assets");
    let models_out = assets_out.join("models");
    let textures_out = assets_out.join("textures");
    std::fs::create_dir_all(&models_out).unwrap();
    std::fs::create_dir_all(&textures_out).unwrap();

    std::fs::copy(&assets_hjson_src, assets_out.join("assets.hjson"))
        .expect("failed to copy assets.hjson to output");

    for (_name, entry) in &assets.models {
        let dir = find_asset_dir(&assets_submodule, &entry.file);
        copy_assets_by_stem(&dir.join(&entry.file), &models_out);
    }

    for (_name, entry) in &assets.textures {
        let dir = find_asset_dir(&assets_submodule, &entry.file);
        copy_assets_by_stem(&dir.join(&entry.file), &textures_out);
    }
}

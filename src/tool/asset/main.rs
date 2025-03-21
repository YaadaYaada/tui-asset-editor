use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

const ROOT_DIRECTORY: &str = "rs";
const ASEPRITE_ASSETS_DIRECTORY: &str = "../gd/asset/sprite/aseprite";

fn main() {
    let root = env::current_dir().unwrap();
    if root.file_stem().unwrap() != ROOT_DIRECTORY {
        panic!("Unable to run from your current directory. Please run this tool from the project's root directory: {:?}", ROOT_DIRECTORY);
    }
    assert_aseprite_in_path();
    convert_aseprite_assets_to_png(root.clone());
}

// Clones and converts all .aseprite files in `asset/aseprite` directory to identically named files
// and folders in `assets`. E.G.
// `asset/sprite/aseprite/icon/cheese.aseprite` would be converted to `asset/sprite/icon/cheese.png`
fn convert_aseprite_assets_to_png(mut root: PathBuf) {
    root.push(ASEPRITE_ASSETS_DIRECTORY);
    let aseprite_files = walk_dir(root.clone()).unwrap();
    let mut png_files: Vec<PathBuf> = vec![];
    println!("Converting aseprite files to png...");
    for aseprite_file in aseprite_files.iter() {
        let mut png_file = PathBuf::from(&aseprite_file.to_str().unwrap().replace("aseprite/", ""));
        png_file.set_extension("png");
        png_files.push(png_file.clone());

        Command::new("aseprite")
            .args([
                "-b",
                "--sheet-type",
                "rows",
                aseprite_file.to_str().unwrap(),
                "--sheet",
                png_file.to_str().unwrap(),
            ])
            .output()
            .expect("failed to run aseprite");

        println!("{:?} -> {:?}", aseprite_file, png_file);
    }
}

// Recursively walks a directory and returns every single non directory file path
// as a Vec of PathBufs.
fn walk_dir(root: PathBuf) -> Option<Vec<PathBuf>> {
    let mut dirs: Vec<PathBuf> = vec![root];
    let mut files: Vec<PathBuf> = vec![];
    while !dirs.is_empty() {
        let current_dir = dirs.pop()?;
        for entry in fs::read_dir(current_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if entry.metadata().unwrap().is_dir() {
                dirs.push(path);
            } else {
                files.push(path);
            }
        }
    }
    Some(files)
}

// Checks if aseprite is in the PATH env.
fn assert_aseprite_in_path() {
    let out = Command::new("which")
        .args(["aseprite"])
        .output()
        .expect("failed to check if aseprite exists in PATH");

    if !out.status.success() {
        panic!("Aseprite not present in path: {:?}", out);
    }
}

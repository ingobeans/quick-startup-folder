#![windows_subsystem = "windows"]

use std::env;
use std::fs;

fn main() {
    let exe_path = env::current_exe().expect("failed getting own path");
    let parent = exe_path.parent().unwrap();
    let startup_path = parent.join("startup");
    let startup_path = startup_path.to_string_lossy().to_string();

    let paths = fs::read_dir(startup_path).unwrap();

    for entry in paths.flatten() {
        let path = entry.path();
        let Some(path_string) = path.to_str() else {
            continue;
        };
        opener::open(path_string).unwrap();
    }
}

use std::fs;
use std::process::Command;

fn main() {
    let paths = fs::read_dir("startup").unwrap();

    for entry in paths {
        let path = entry.unwrap().path();
        Command::new("cmd").args(["/C", "start", path.to_str().unwrap()]).spawn().unwrap();
    }
}

use std::fs;
use std::process::Command;

fn main() {
    let paths = fs::read_dir("startup").unwrap();

    for entry in paths {
        let path = entry.unwrap().path();
        let quotes_path = String::from("'") + path.to_str().unwrap() + "'";
        let t = ["/C", "start", &quotes_path];
        Command::new("powershell").args(t).spawn().unwrap();
    }
}

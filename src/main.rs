use std::fs;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let dir_path = "startup";

    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            Command::new("cmd").args(&["/C", "start", "", path.to_str().unwrap()]).spawn()?;
        }
    }

    Ok(())
}

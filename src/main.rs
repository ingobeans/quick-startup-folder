#![windows_subsystem = "windows"]

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process::Command;
mod has_admin;

fn run_command(args: &[&str]) -> bool {
    let output = Command::new("cmd").args(args).output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn task_exists() -> bool {
    run_command(&["/C", "schtasks", "/Query", "/TN", "QuickStartupFolder"])
}

fn create_task() -> bool {
    let exe_path = env::current_exe().expect("failed getting path to exe");
    let exe_path = exe_path.to_string_lossy();
    run_command(&[
        "/C",
        "schtasks",
        "/create",
        "/tn",
        "QuickStartupFolder",
        "/tr",
        &format!("{exe_path} run"),
        "/sc",
        "onlogon",
        "/ru",
        "%USERDOMAIN%\\%USERNAME%",
        "/f",
    ])
}

fn get_yn_prompt(text: &str) -> bool {
    print!("{text} (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();

    return match input.as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => false,
    };
}

fn setup_task() {
    let task_exists = task_exists();
    if !task_exists {
        println!("quick startup folder is not a scheduled task.");
    } else {
        println!(
            "quick startup folder is already a scheduled task, doing this will update/reset the task."
        )
    }
    let admin = has_admin::is_elevated();
    if !admin {
        println!("you gotta run this with admin to setup scheduled task!");
        dont_disappear::any_key_to_continue::custom_msg("press any key to close...");
        return;
    }
    let input = get_yn_prompt("do you want to add this as a scheduled task?");
    if input {
        println!("attempting to schedule...");
        let result = create_task();
        if result {
            println!("success! quick startup folder is now setup and will run everything from the startup folder on login!")
        } else {
            println!("error: couldn't schedule task!")
        }
        dont_disappear::any_key_to_continue::custom_msg("press any key to close...");
    } else {
        println!("ok fine..");
        dont_disappear::any_key_to_continue::custom_msg("press any key to close...");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "run" {
        let exe_path = env::current_exe().expect("failed getting path to exe");
        let exe_dir = exe_path.parent().expect("failed to get parent directory");
        let startup_path = exe_dir.join("startup");
        let startup_path = startup_path.to_string_lossy().to_string();

        let paths = fs::read_dir(startup_path).unwrap();

        for entry in paths {
            let path = entry.unwrap().path();
            opener::open(path.to_str().unwrap()).unwrap();
        }
    } else {
        setup_task();
    }
}

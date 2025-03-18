use std::env;
use std::io;
use std::io::Write;
use std::process::Command;

#[path = "../has_admin.rs"]
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

fn get_path_to_binary() -> String {
    let self_path = env::current_exe().expect("failed getting own path");
    let parent = self_path.parent().unwrap();
    let binary_path = parent.join("quick-startup-folder.exe");
    if !binary_path.exists() {
        println!("error: 'quick-startup-folder.exe' is not present in the same directory");
        std::process::exit(0);
    }
    binary_path.to_string_lossy().to_string()
}

fn create_task() -> bool {
    let exe_path = get_path_to_binary();
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

fn dont_exit() {
    dont_disappear::any_key_to_continue::custom_msg("press any key to close...");
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
        dont_exit();
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
        dont_exit();
    } else {
        println!("ok fine..");
        dont_exit();
    }
}

fn main() {
    setup_task();
}

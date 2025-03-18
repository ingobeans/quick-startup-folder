use std::env;
use std::io;
use std::io::stdout;
use std::io::Write;
use std::process::Command;

use crossterm::queue;
use crossterm::style::Color;
use crossterm::style::SetForegroundColor;

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
fn remove_task() -> bool {
    run_command(&[
        "/C",
        "schtasks",
        "/delete",
        "/tn",
        "QuickStartupFolder",
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

fn dont_exit() -> ! {
    dont_disappear::any_key_to_continue::custom_msg("press any key to close...");
    std::process::exit(0);
}

fn main() {
    let task_exists = task_exists();
    print!("* quick-startup-folder status: ",);
    if task_exists {
        queue!(stdout(), SetForegroundColor(Color::Green)).unwrap();
        println!("SCHEDULED")
    } else {
        queue!(stdout(), SetForegroundColor(Color::Red)).unwrap();
        println!("NOT SCHEDULED")
    }
    queue!(stdout(), SetForegroundColor(Color::Reset)).unwrap();
    let admin = has_admin::is_elevated();
    print!("* has admin: ");
    if admin {
        queue!(stdout(), SetForegroundColor(Color::Green)).unwrap();
        println!("YES")
    } else {
        queue!(stdout(), SetForegroundColor(Color::Red)).unwrap();
        println!("NO")
    }
    println!("");
    queue!(stdout(), SetForegroundColor(Color::Reset)).unwrap();
    if !admin {
        println!("this tool needs admin to run");
        dont_exit();
    }
    let success: bool;
    if !task_exists {
        let input = get_yn_prompt("add quick-startup-folder as scheduled task?");
        if input {
            success = create_task();
        } else {
            return;
        }
    } else {
        let input = get_yn_prompt("remove quick-startup-folder as scheduled task?");
        if input {
            success = remove_task();
        } else {
            return;
        }
    }
    if success {
        println!("success!")
    } else {
        println!("something failed :<")
    }
    dont_exit();
}

use std::{env, path::Path, process::Command};

pub fn execute_command(command: Vec<&str>, should_exit: &mut bool) {
    if command.is_empty() {
        return;
    }

    if command[0] == "exit" && command.len() == 1 {
        *should_exit = true;
        return;
    }

    // Handle the cd command
    if command[0] == "cd" {
        handle_cd(&command);
        return;
    }

    run_external_command(command);
}

fn handle_cd(command: &Vec<&str>) {
    let new_dir = if command.len() > 1 {
        command[1].to_string()
    } else {
        // Default to home directory if no path specified
        match env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
            Ok(home) => home,
            Err(_) => {
                eprintln!("Could not determine home directory");
                return; // Return early if we can't get home directory
            }
        }
    };

    // Change the working directory
    match env::set_current_dir(Path::new(&new_dir)) {
        Ok(_) => {
            if let Ok(dir) = env::current_dir() {
                println!("Changed directory to: {}", dir.display());
            }
        }
        Err(e) => eprintln!("Failed to change directory: {}", e),
    }
}

fn run_external_command(command: Vec<&str>) {
    match Command::new(command[0]).args(&command[1..]).spawn() {
        Ok(mut child) => match child.wait() {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Command exited with status: {}", status);
                }
            }
            Err(e) => eprintln!("Failed to wait for command: {}", e),
        },
        Err(e) => eprintln!("Failed to execute command: {}", e),
    };
}

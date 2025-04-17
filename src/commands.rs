use std::process::Command;

pub fn execute_command(command: Vec<&str>, should_exit: &mut bool) {
    if command[0] == "exit" && command.len() == 1 {
        *should_exit = true;
        return;
    }

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

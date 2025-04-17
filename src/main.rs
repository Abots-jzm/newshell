use std::{
    env::args,
    fs,
    io::{stdout, Write},
};

mod commands;

use commands::execute_command;

enum Mode {
    Interactive,
    Batch,
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        eprintln!("Usage: {} [bactchFile]", args[0]);
        std::process::exit(1);
    }

    let mode = if args.len() == 2 {
        Mode::Batch
    } else {
        Mode::Interactive
    };

    let mut should_exit = false;

    let r = ctrlc::set_handler(move || {
        println!("\nExiting shell...");
        std::process::exit(0);
    });

    if let Err(err) = r {
        eprintln!("Error setting Ctrl+C handler: {}", err);
    }

    while !should_exit {
        match mode {
            Mode::Interactive => {
                print!("--SHELL-> ");
                stdout().flush().unwrap();
                let mut input = String::new();
                if std::io::stdin().read_line(&mut input).is_ok() {
                    let input = input.trim();
                    if !input.is_empty() {
                        let commands = tokenize_input(input);
                        for command in commands {
                            execute_command(command, &mut should_exit);
                        }
                    } else {
                        should_exit = true;
                    }
                } else {
                    should_exit = true;
                }
            }
            Mode::Batch => {
                let content = match fs::read_to_string(&args[1]) {
                    Ok(content) => content,
                    Err(err) => {
                        eprintln!("[ERROR] Unable to open batchFile: {} ", err);
                        std::process::exit(1);
                    }
                };
                for line in content.lines() {
                    if should_exit {
                        break;
                    }

                    print!("[BATCH] ");
                    stdout().flush().unwrap();
                    let input = line.trim();
                    if !input.is_empty() {
                        let commands = tokenize_input(input);
                        for command in commands {
                            execute_command(command, &mut should_exit);
                        }
                    } else {
                        should_exit = true;
                    }
                }
            }
        }
    }
}

fn tokenize_input(input: &str) -> Vec<Vec<&str>> {
    input
        .split(';')
        .map(|x| {
            x.split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim())
                .collect()
        })
        .collect()
}

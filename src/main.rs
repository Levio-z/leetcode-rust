#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod fetcher;
mod cli;

use std::io;

use cli::{execute_command, get_initialized_problem_ids, parse_command};

/// main() helps to generate the submission template .rs
fn main() {
    println!("Welcome to leetcode-rust system.\n");
    let initialized_ids = get_initialized_problem_ids();

    loop {
        println!(
            "Please enter a frontend problem id, \n\
            or \"random\" to generate a random one, \n\
            or \"solve $i\" to move problem to solution/, \n\
            or \"all\" to initialize all problems \n"
        );

        let mut id_arg = String::new();
        io::stdin()
            .read_line(&mut id_arg)
            .expect("Failed to read line");

        match parse_command(&id_arg) {
            Ok(cmd) => match execute_command(cmd, &initialized_ids) {
                Ok(should_exit) => {
                    if should_exit {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Error parsing command: {}", e);
            }
        }
    }
}

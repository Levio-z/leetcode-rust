#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod cli;
mod fetcher;

use clap::Parser;
use cli::{Command, execute_command, get_initialized_problem_ids};

/// LeetCode Rust CLI - Generate and manage LeetCode problem templates
#[derive(Parser)]
#[command(name = "lc")]
#[command(about = "LeetCode Rust problem generator", long_about = None)]
struct Cli {
    /// Problem ID to fetch (e.g., 01, 42, 100)
    problem_id: Option<String>,

    /// Generate a random problem
    #[arg(short = 'r', long = "random")]
    random: bool,

    /// Move problem to solution directory
    #[arg(short = 's', long = "solve")]
    solve: bool,

    /// Initialize all problems
    #[arg(short = 'a', long = "all")]
    all: bool,
}

/// main() helps to generate the submission template .rs
fn main() {
    // 初始化tracing订阅者
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let initialized_ids = get_initialized_problem_ids();

    let command = if cli.all {
        Command::All
    } else if cli.random {
        Command::Random
    } else if let Some(id_str) = cli.problem_id {
        if cli.solve {
            // Parse the problem ID for solve command
            let id = id_str.parse::<u32>().unwrap_or_else(|_| {
                eprintln!("Error: Invalid problem ID: {}", id_str);
                std::process::exit(1);
            });
            Command::Solve(id)
        } else {
            // Parse the problem ID for fetch command
            let id = id_str.parse::<u32>().unwrap_or_else(|_| {
                eprintln!("Error: Invalid problem ID: {}", id_str);
                std::process::exit(1);
            });
            Command::FetchById(id)
        }
    } else {
        eprintln!("Error: Please provide a problem ID or use --random/--all");
        std::process::exit(1);
    };

    match execute_command(command, &initialized_ids) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

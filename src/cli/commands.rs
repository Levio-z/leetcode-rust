use std::{
    fs,
    sync::{Arc, Mutex},
};

use futures::{executor::block_on, future::join_all, task::SpawnExt};
use rand::Rng;
use regex::Regex;

use super::handlers::{create_problem_file, get_initialized_problem_ids, move_to_solution};
use crate::fetcher;

/// Command types supported by the CLI
#[derive(Debug)]
pub enum Command {
    Random,
    Solve(u32),
    All,
    FetchById(u32),
}

/// Parse user input into a Command
pub fn parse_command(input: &str) -> Result<Command, String> {
    let input = input.trim();

    let random_pattern = Regex::new(r"^random$").unwrap();
    let solving_pattern = Regex::new(r"^solve (\d+)$").unwrap();
    let all_pattern = Regex::new(r"^all$").unwrap();

    if random_pattern.is_match(input) {
        Ok(Command::Random)
    } else if let Some(caps) = solving_pattern.captures(input) {
        let id = caps
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .map_err(|_| "Invalid problem ID")?;
        Ok(Command::Solve(id))
    } else if all_pattern.is_match(input) {
        Ok(Command::All)
    } else {
        let id = input
            .parse::<u32>()
            .map_err(|_| format!("not a number: {}", input))?;
        Ok(Command::FetchById(id))
    }
}

/// Generate a random problem ID that hasn't been initialized
fn generate_random_id(except_ids: &[u32]) -> u32 {
    let mut rng = rand::rng();
    loop {
        let res: u32 = rng.random_range(1..1106);
        if !except_ids.contains(&res) {
            return res;
        }
        println!(
            "Generate a random num ({}), but it is invalid (the problem may have been solved \
             or may have no rust version). Regenerate..",
            res
        );
    }
}

/// Execute a parsed command
pub fn execute_command(cmd: Command, initialized_ids: &[u32]) -> Result<bool, String> {
    match cmd {
        Command::Random => {
            println!("You select random mode.");
            let id = generate_random_id(initialized_ids);
            println!("Generate random problem: {}", id);

            let problem = fetcher::get_problem(id).ok_or_else(|| {
                format!(
                    "Error: failed to get problem #{} \
                     (The problem may be paid-only or may not exist).",
                    id
                )
            })?;

            let code = problem
                .code_definition
                .iter()
                .find(|&d| d.value == "rust".to_string())
                .ok_or_else(|| format!("Problem {} has no rust version.", id))?;

            create_problem_file(&problem, code, true);
            Ok(true) // Exit after processing
        }
        Command::Solve(id) => {
            move_to_solution(&id);
            Ok(true) // Exit after processing
        }
        Command::All => {
            execute_all_command(initialized_ids)?;
            Ok(true) // Exit after processing
        }
        Command::FetchById(id) => {
            if initialized_ids.contains(&id) {
                println!("The problem you chose has been initialized in problem/");
                return Ok(false); // Continue loop
            }

            let problem = fetcher::get_problem(id).ok_or_else(|| {
                format!(
                    "Error: failed to get problem #{} \
                     (The problem may be paid-only or may not exist).",
                    id
                )
            })?;

            let code = problem
                .code_definition
                .iter()
                .find(|&d| d.value == "rust".to_string());

            if code.is_none() {
                println!("Problem {} has no rust version.", id);
                return Ok(false); // Continue loop
            }

            create_problem_file(&problem, code.unwrap(), true);
            Ok(true) // Exit after processing
        }
    }
}

/// Execute the "all" command to initialize all problems
fn execute_all_command(initialized_ids: &[u32]) -> Result<(), String> {
    let pool = futures::executor::ThreadPool::new()
        .map_err(|e| format!("Failed to create thread pool: {}", e))?;
    let mut tasks = vec![];
    let problems = fetcher::get_problems().ok_or("Failed to fetch problems")?;
    let mod_file_addon = Arc::new(Mutex::new(vec![]));

    for problem_stat in problems.stat_status_pairs {
        if initialized_ids.contains(&problem_stat.stat.frontend_question_id) {
            continue;
        }
        let mod_file_addon = mod_file_addon.clone();
        tasks.push(
            pool.spawn_with_handle(async move {
                let problem = fetcher::get_problem_async(problem_stat).await;
                if problem.is_none() {
                    return;
                }
                let problem = problem.unwrap();
                let code = problem
                    .code_definition
                    .iter()
                    .find(|&d| d.value == "rust".to_string());
                if code.is_none() {
                    println!("Problem {} has no rust version.", problem.question_id);
                    return;
                }
                async {
                    mod_file_addon.lock().unwrap().push(format!(
                        "mod p{:04}_{};",
                        problem.question_id,
                        problem.title_slug.replace("-", "_")
                    ));
                }
                .await;
                let code = code.unwrap();
                async { create_problem_file(&problem, &code, false) }.await
            })
            .map_err(|e| format!("Failed to spawn task: {}", e))?,
        );
    }
    block_on(join_all(tasks));

    let mut lib_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("./src/problem/mod.rs")
        .map_err(|e| format!("Failed to open mod.rs: {}", e))?;

    use std::io::Write;
    writeln!(lib_file, "{}", mod_file_addon.lock().unwrap().join("\n"))
        .map_err(|e| format!("Failed to write to mod.rs: {}", e))?;

    Ok(())
}

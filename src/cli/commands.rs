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
    use std::io::Write;

    println!("Fetching problem lists...");

    // Fetch both problem lists concurrently
    let pool = futures::executor::ThreadPool::new()
        .map_err(|e| format!("Failed to create thread pool: {}", e))?;

    let problems_future = pool.spawn_with_handle(async {
        fetcher::get_problems()
    }).map_err(|e| format!("Failed to spawn task: {}", e))?;

    let concurrency_future = pool.spawn_with_handle(async {
        fetcher::get_concurrency()
    }).map_err(|e| format!("Failed to spawn task: {}", e))?;

    let (problems, concurrency_problems) = block_on(async {
        let p = problems_future.await;
        let c = concurrency_future.await;
        (p, c)
    });

    let problems = problems.ok_or("Failed to fetch algorithm problems")?;
    let concurrency_problems = concurrency_problems.ok_or("Failed to fetch concurrency problems")?;

    // Collect all problems to process
    let all_problems: Vec<_> = problems.stat_status_pairs
        .into_iter()
        .chain(concurrency_problems.stat_status_pairs.into_iter())
        .filter(|p| !initialized_ids.contains(&p.stat.frontend_question_id))
        .collect();

    let total = all_problems.len();
    println!("Found {} problems to initialize", total);

    if total == 0 {
        println!("All problems are already initialized!");
        return Ok(());
    }

    let mod_file_addon = Arc::new(Mutex::new(vec![]));
    let completed = Arc::new(Mutex::new(0usize));

    // Process in batches to control concurrency and show progress
    const BATCH_SIZE: usize = 50;
    let batches: Vec<_> = all_problems.chunks(BATCH_SIZE).collect();
    let num_batches = batches.len();

    println!("Initializing problems in {} batches (batch size: {})...", num_batches, BATCH_SIZE);

    for (batch_idx, batch) in batches.into_iter().enumerate() {
        println!("Processing batch {}/{}...", batch_idx + 1, num_batches);

        let mut tasks = vec![];

        for problem_stat in batch {
            let mod_file_addon = mod_file_addon.clone();
            let completed = completed.clone();
            let problem_stat = problem_stat.clone();

            tasks.push(
                pool.spawn_with_handle(async move {
                    let problem = fetcher::get_problem_async(problem_stat).await;

                    if let Some(problem) = problem {
                        if let Some(code) = problem.code_definition.iter().find(|&d| d.value == "rust") {
                            mod_file_addon.lock().unwrap().push(format!(
                                "mod p{:04}_{};",
                                problem.question_id,
                                problem.title_slug.replace("-", "_")
                            ));

                            create_problem_file(&problem, code, false);

                            let mut count = completed.lock().unwrap();
                            *count += 1;
                        } else {
                            println!("Problem {} has no rust version.", problem.question_id);
                        }
                    }
                })
                .map_err(|e| format!("Failed to spawn task: {}", e))?,
            );
        }

        block_on(join_all(tasks));

        let current_count = *completed.lock().unwrap();
        println!("Progress: {}/{} problems initialized", current_count, total);
    }

    let mut lib_file = fs::OpenOptions::new()
        .append(true)
        .open("./src/problem/mod.rs")
        .map_err(|e| format!("Failed to open mod.rs: {}", e))?;

    writeln!(lib_file, "{}", mod_file_addon.lock().unwrap().join("\n"))
        .map_err(|e| format!("Failed to write to mod.rs: {}", e))?;

    println!("Successfully initialized {} problems!", completed.lock().unwrap());
    Ok(())
}

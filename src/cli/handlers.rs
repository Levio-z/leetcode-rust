use std::{
    fs,
    io::{self, BufRead, Write},
    path::Path,
};

use regex::Regex;

use super::template::{
    clean_html_description, format_discuss_url, format_problem_url, generate_extra_imports,
    insert_default_return,
};
use crate::fetcher::{CodeDefinition, Problem};

/// Get list of initialized problem IDs from problem/mod.rs
pub fn get_initialized_problem_ids() -> Vec<u32> {
    let content = fs::read_to_string("./src/problem/mod.rs").unwrap();
    let id_pattern = Regex::new(r"p(\d{4})_").unwrap();
    id_pattern
        .captures_iter(&content)
        .map(|x| x.get(1).unwrap().as_str().parse().unwrap())
        .collect()
}

/// Find problem file name by ID
pub fn find_problem_file(id: &u32) -> String {
    fs::read_dir("./src/problem")
        .map(|rd| {
            Box::new(rd.filter_map(|x| x.ok()).filter_map(|x| {
                let file_name = x.file_name().into_string().ok()?;
                if file_name.starts_with(&format!("p{:04}", id)) {
                    Some(file_name.strip_suffix(".rs").unwrap().to_string())
                } else {
                    None
                }
            })) as Box<dyn Iterator<Item = String>>
        })
        .unwrap_or_else(|_| Box::new(std::iter::empty()))
        .next()
        .unwrap_or_default()
}

/// Create a problem file from fetched problem data
pub fn create_problem_file(problem: &Problem, code: &CodeDefinition, write_mod_file: bool) {
    let file_name = format!(
        "p{:04}_{}",
        problem.question_id,
        problem.title_slug.replace("-", "_")
    );
    let file_path = Path::new("./src/problem").join(format!("{}.rs", file_name));
    if file_path.exists() {
        panic!("problem already initialized");
    }

    let template = fs::read_to_string("./template.rs").unwrap();
    let source = template
        .replace("__PROBLEM_TITLE__", &problem.title)
        .replace(
            "__PROBLEM_DESC__",
            &clean_html_description(&problem.content),
        )
        .replace(
            "__PROBLEM_DEFAULT_CODE__",
            &insert_default_return(&problem.return_type, &code.default_code),
        )
        .replace("__PROBLEM_ID__", &format!("{}", problem.question_id))
        .replace("__EXTRA_USE__", &generate_extra_imports(&code.default_code))
        .replace("__PROBLEM_LINK__", &format_problem_url(problem))
        .replace("__DISCUSS_LINK__", &format_discuss_url(problem));

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .unwrap();

    file.write_all(source.as_bytes()).unwrap();
    drop(file);

    if write_mod_file {
        let mut lib_file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("./src/problem/mod.rs")
            .unwrap();
        writeln!(lib_file, "mod {};", file_name).unwrap();
    }
}

/// Move a problem file to solution directory
pub fn move_to_solution(id: &u32) {
    let file_name = find_problem_file(id);

    let file_path = Path::new("./src/problem").join(format!("{}.rs", file_name));
    // check problem/ existence
    if !file_path.exists() {
        panic!("problem does not exist");
    }
    // check solution/ no existence
    let solution_name = file_name
        .strip_prefix('p')
        .map(|rest| format!("s{}", rest))
        .expect("file name must start with p");
    let solution_path = Path::new("./src/solution").join(format!("{}.rs", solution_name));
    if solution_path.exists() {
        panic!("solution exists");
    }
    // rename/move file
    fs::rename(file_path, solution_path).unwrap();
    // remove from problem/mod.rs
    let mod_file = "./src/problem/mod.rs";
    let target_line = format!("mod {};", file_name);
    let lines: Vec<String> = io::BufReader::new(fs::File::open(mod_file).unwrap())
        .lines()
        .map(|x| {
            let mut x = x.unwrap();
            x.replace(&target_line, "");
            x
        })
        .collect();
    fs::write(mod_file, lines.join("\n")).unwrap();
    // insert into solution/mod.rs
    let mut lib_file = fs::OpenOptions::new()
        .append(true)
        .open("./src/solution/mod.rs")
        .unwrap();
    writeln!(lib_file, "mod {};", solution_name).unwrap();
}

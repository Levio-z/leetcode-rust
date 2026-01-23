extern crate reqwest;

use super::{
    api_models::{ProblemsResponse, Query, RawProblem},
    models::{Problem, Problems, StatWithStatus},
    transform::{convert_to_public_stat, transform_raw_problem, transform_raw_problem_async},
};

const PROBLEMS_URL: &str = "https://leetcode.com/api/problems/algorithms/";
const PROBLEMS_CONCURRENCY_URL: &str = "https://leetcode.com/api/problems/concurrency/";
const GRAPHQL_URL: &str = "https://leetcode.com/graphql";

/// Fetch a specific problem by its frontend question ID (blocking)
pub fn get_problem(frontend_question_id: u32) -> Option<Problem> {
    let problems = get_problems()?;
    let concurrency_problems = get_concurrency()?;

    for problem in problems
        .stat_status_pairs
        .iter()
        .chain(concurrency_problems.stat_status_pairs.iter())
    {
        if problem.stat.frontend_question_id == frontend_question_id {
            if problem.paid_only {
                return None;
            }

            let client = reqwest::blocking::Client::new();
            let resp: RawProblem = client
                .post(GRAPHQL_URL)
                .json(&Query::question_query(
                    problem.stat.question_title_slug.as_ref()?,
                ))
                .send()
                .ok()?
                .json()
                .ok()?;

            return Some(Problem {
                title: problem.stat.question_title.clone().unwrap(),
                title_slug: problem.stat.question_title_slug.clone().unwrap(),
                code_definition: serde_json::from_str(&resp.data.question.code_definition).unwrap(),
                content: resp.data.question.content,
                sample_test_case: resp.data.question.sample_test_case,
                difficulty: problem.difficulty.to_string(),
                question_id: problem.stat.frontend_question_id,
                return_type: {
                    let v: serde_json::Value =
                        serde_json::from_str(&resp.data.question.meta_data).unwrap();
                    v["return"]["type"].to_string().replace("\"", "")
                },
            });
        }
    }
    None
}

/// Fetch a specific problem asynchronously
pub async fn get_problem_async(problem_stat: StatWithStatus) -> Option<Problem> {
    if problem_stat.paid_only {
        println!(
            "Problem {} is paid-only",
            &problem_stat.stat.frontend_question_id
        );
        return None;
    }

    let resp = surf::post(GRAPHQL_URL)
        .body_json(&Query::question_query(
            problem_stat.stat.question_title_slug.as_ref()?,
        ))
        .ok()?
        .recv_json()
        .await;

    if resp.is_err() {
        println!(
            "Problem {} not initialized due to some error",
            &problem_stat.stat.frontend_question_id
        );
        return None;
    }

    let resp: RawProblem = resp.ok()?;

    transform_raw_problem_async(
        resp,
        problem_stat.stat.question_title.clone()?,
        problem_stat.stat.question_title_slug.clone()?,
        problem_stat.difficulty_string(),
        problem_stat.stat.frontend_question_id,
    )
    .ok()
}

/// Fetch all algorithm problems (blocking)
pub fn get_problems() -> Option<Problems> {
    reqwest::blocking::get(PROBLEMS_URL).ok()?.json().ok()?
}

/// Fetch all concurrency problems (blocking)
pub fn get_concurrency() -> Option<Problems> {
    reqwest::blocking::get(PROBLEMS_CONCURRENCY_URL)
        .ok()?
        .json()
        .ok()?
}

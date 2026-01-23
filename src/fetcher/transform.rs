use serde_json::Value;

use super::api_models::{RawProblem, StatWithStatusInternal};
use super::models::{Problem, StatWithStatus};

/// Transform raw API problem response into domain Problem model
pub(crate) fn transform_raw_problem(
    raw: RawProblem,
    stat: &StatWithStatusInternal,
) -> Result<Problem, String> {
    let code_definition = serde_json::from_str(&raw.data.question.code_definition)
        .map_err(|e| format!("Failed to parse code definition: {}", e))?;

    let return_type = extract_return_type(&raw.data.question.meta_data)?;

    Ok(Problem {
        title: stat
            .stat
            .question_title
            .clone()
            .ok_or("Missing question title")?,
        title_slug: stat
            .stat
            .question_title_slug
            .clone()
            .ok_or("Missing title slug")?,
        code_definition,
        content: raw.data.question.content,
        sample_test_case: raw.data.question.sample_test_case,
        difficulty: stat.difficulty.to_string(),
        question_id: stat.stat.frontend_question_id,
        return_type,
    })
}

/// Transform raw API problem response for async version
pub(crate) fn transform_raw_problem_async(
    raw: RawProblem,
    title: String,
    title_slug: String,
    difficulty: String,
    question_id: u32,
) -> Result<Problem, String> {
    let code_definition = serde_json::from_str(&raw.data.question.code_definition)
        .map_err(|e| format!("Failed to parse code definition: {}", e))?;

    let return_type = extract_return_type(&raw.data.question.meta_data)?;

    Ok(Problem {
        title,
        title_slug,
        code_definition,
        content: raw.data.question.content,
        sample_test_case: raw.data.question.sample_test_case,
        difficulty,
        question_id,
        return_type,
    })
}

/// Extract return type from metadata JSON
fn extract_return_type(meta_data: &str) -> Result<String, String> {
    let v: Value = serde_json::from_str(meta_data)
        .map_err(|e| format!("Failed to parse metadata: {}", e))?;

    Ok(v["return"]["type"]
        .to_string()
        .replace("\"", ""))
}

/// Convert internal API model to public domain model
pub(crate) fn convert_to_public_stat(internal: StatWithStatusInternal) -> StatWithStatus {
    // This is a workaround since the structs are identical
    // In a real scenario, we'd manually construct the public version
    let json = serde_json::to_string(&internal).unwrap();
    serde_json::from_str(&json).unwrap()
}

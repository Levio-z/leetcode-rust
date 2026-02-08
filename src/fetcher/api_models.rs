use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// GraphQL query structure for LeetCode API
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Query {
    #[serde(rename = "operationName")]
    pub operation_name: String,
    pub variables: Value,
    pub query: String,
}

impl Query {
    pub fn question_query(title_slug: &str) -> Query {
        const QUESTION_QUERY_STRING: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
        stats
        codeDefinition
        sampleTestCase
        metaData
    }
}"#;
        const QUESTION_QUERY_OPERATION: &str = "questionData";

        Query {
            operation_name: QUESTION_QUERY_OPERATION.to_owned(),
            variables: json!({ "titleSlug": title_slug }),
            query: QUESTION_QUERY_STRING.to_owned(),
        }
    }
}

/// Raw problem response from GraphQL API
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RawProblem {
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Data {
    pub question: Question,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Question {
    pub content: String,
    pub stats: String,
    #[serde(rename = "codeDefinition")]
    pub code_definition: String,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    #[serde(rename = "metaData")]
    pub meta_data: String,
}

/// Problems list response from REST API
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProblemsResponse {
    pub user_name: String,
    pub num_solved: u32,
    pub num_total: u32,
    pub ac_easy: u32,
    pub ac_medium: u32,
    pub ac_hard: u32,
    pub stat_status_pairs: Vec<StatWithStatusInternal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct StatWithStatusInternal {
    pub stat: StatInternal,
    pub difficulty: Difficulty,
    pub paid_only: bool,
    pub is_favor: bool,
    pub frequency: u32,
    pub progress: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct StatInternal {
    pub question_id: u32,
    #[serde(rename = "question__article__slug")]
    pub question_article_slug: Option<String>,
    #[serde(rename = "question__title")]
    pub question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    pub question_title_slug: Option<String>,
    #[serde(rename = "question__hide")]
    pub question_hide: bool,
    pub total_acs: u32,
    pub total_submitted: u32,
    pub frontend_question_id: u32,
    pub is_new_question: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Difficulty {
    pub level: u32,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let difficulty = match self.level {
            1 => "Easy".to_string(),
            2 => "Medium".to_string(),
            3 => "Hard".to_string(),
            _ => "Unknown".to_string(),
        };
        write!(f, "{}", difficulty)
    }
}

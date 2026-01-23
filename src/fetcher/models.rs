use std::fmt::{Display, Error, Formatter};

use serde::{Deserialize, Serialize};

/// Public domain model for a LeetCode problem
#[derive(Serialize, Deserialize)]
pub struct Problem {
    pub title: String,
    pub title_slug: String,
    pub content: String,
    #[serde(rename = "codeDefinition")]
    pub code_definition: Vec<CodeDefinition>,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    pub difficulty: String,
    pub question_id: u32,
    pub return_type: String,
}

/// Code definition for a specific language
#[derive(Serialize, Deserialize)]
pub struct CodeDefinition {
    pub value: String,
    pub text: String,
    #[serde(rename = "defaultCode")]
    pub default_code: String,
}

/// Collection of problems
#[derive(Debug, Serialize, Deserialize)]
pub struct Problems {
    pub user_name: String,
    pub num_solved: u32,
    pub num_total: u32,
    pub ac_easy: u32,
    pub ac_medium: u32,
    pub ac_hard: u32,
    pub stat_status_pairs: Vec<StatWithStatus>,
}

/// Problem statistics with status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatWithStatus {
    pub stat: Stat,
    pub difficulty: Difficulty,
    pub paid_only: bool,
    is_favor: bool,
    frequency: u32,
    progress: u32,
}

impl StatWithStatus {
    pub fn difficulty_string(&self) -> String {
        self.difficulty.to_string()
    }
}

/// Problem statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    question_id: u32,
    #[serde(rename = "question__article__slug")]
    question_article_slug: Option<String>,
    #[serde(rename = "question__title")]
    pub question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    pub question_title_slug: Option<String>,
    #[serde(rename = "question__hide")]
    question_hide: bool,
    total_acs: u32,
    total_submitted: u32,
    pub frontend_question_id: u32,
    is_new_question: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Difficulty {
    level: u32,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.level {
            1 => f.write_str("Easy"),
            2 => f.write_str("Medium"),
            3 => f.write_str("Hard"),
            _ => f.write_str("Unknown"),
        }
    }
}

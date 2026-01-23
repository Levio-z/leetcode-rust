use regex::Regex;
use std::collections::HashMap;

use crate::fetcher::Problem;

/// Generate extra import statements based on code content
pub fn generate_extra_imports(code: &str) -> String {
    let mut extra_use_line = String::new();

    if code.contains("pub struct ListNode") {
        extra_use_line.push_str("\nuse crate::util::linked_list::{ListNode, to_list};")
    }
    if code.contains("pub struct TreeNode") {
        extra_use_line.push_str("\nuse crate::util::tree::{TreeNode, to_tree};")
    }
    if code.contains("pub struct Point") {
        extra_use_line.push_str("\nuse crate::util::point::Point;")
    }
    extra_use_line
}

/// Format the problem URL
pub fn format_problem_url(problem: &Problem) -> String {
    format!("https://leetcode.com/problems/{}/", problem.title_slug)
}

/// Format the discuss URL
pub fn format_discuss_url(problem: &Problem) -> String {
    format!(
        "https://leetcode.com/problems/{}/discuss/?currentPage=1&orderBy=most_votes&query=",
        problem.title_slug
    )
}

/// Insert default return value based on return type
pub fn insert_default_return(return_type: &str, code: &str) -> String {
    let re = Regex::new(r"\{[ \n]+}").unwrap();

    let return_map: HashMap<&str, &str> = [
        ("ListNode", "{\n        Some(Box::new(ListNode::new(0)))\n    }"),
        ("ListNode[]", "{\n        vec![]\n    }"),
        ("TreeNode", "{\n        Some(Rc::new(RefCell::new(TreeNode::new(0))))\n    }"),
        ("boolean", "{\n        false\n    }"),
        ("character", "{\n        '0'\n    }"),
        ("character[][]", "{\n        vec![]\n    }"),
        ("double", "{\n        0f64\n    }"),
        ("double[]", "{\n        vec![]\n    }"),
        ("int[]", "{\n        vec![]\n    }"),
        ("integer", "{\n        0\n    }"),
        ("integer[]", "{\n        vec![]\n    }"),
        ("integer[][]", "{\n        vec![]\n    }"),
        ("list<String>", "{\n        vec![]\n    }"),
        ("list<TreeNode>", "{\n        vec![]\n    }"),
        ("list<boolean>", "{\n        vec![]\n    }"),
        ("list<double>", "{\n        vec![]\n    }"),
        ("list<integer>", "{\n        vec![]\n    }"),
        ("list<list<integer>>", "{\n        vec![]\n    }"),
        ("list<list<string>>", "{\n        vec![]\n    }"),
        ("list<string>", "{\n        vec![]\n    }"),
        ("string", "{\n        String::new()\n    }"),
        ("string[]", "{\n        vec![]\n    }"),
    ]
    .iter()
    .copied()
    .collect();

    if let Some(replacement) = return_map.get(return_type) {
        re.replace(code, *replacement).to_string()
    } else {
        code.to_string()
    }
}

/// Clean HTML tags from problem description
pub fn clean_html_description(content: &str) -> String {
    let replacements = [
        ("<strong>", ""),
        ("</strong>", ""),
        ("<em>", ""),
        ("</em>", ""),
        ("</p>", ""),
        ("<p>", ""),
        ("<b>", ""),
        ("</b>", ""),
        ("<pre>", ""),
        ("</pre>", ""),
        ("<ul>", ""),
        ("</ul>", ""),
        ("<li>", ""),
        ("</li>", ""),
        ("<code>", ""),
        ("</code>", ""),
        ("<i>", ""),
        ("</i>", ""),
        ("<sub>", ""),
        ("</sub>", ""),
        ("</sup>", ""),
        ("<sup>", "^"),
        ("&nbsp;", " "),
        ("&gt;", ">"),
        ("&lt;", "<"),
        ("&quot;", "\""),
        ("&minus;", "-"),
        ("&#39;", "'"),
    ];

    let mut result = content.to_string();
    for (from, to) in &replacements {
        result = result.replace(from, to);
    }

    result = result.replace("\n\n", "\n");
    result.replace("\n", "\n * ")
}

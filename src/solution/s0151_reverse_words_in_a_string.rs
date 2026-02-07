/// [151] Reverse Words in a String
///
/// Given an input string s, reverse the order of the words.
/// A word is defined as a sequence of non-space characters. The words in s will
/// be separated by at least one space. Return a string of the words in reverse
/// order concatenated by a single space. Note that s may contain leading or
/// trailing spaces or multiple spaces between two words. The returned string
/// should only have a single space separating the words. Do not include any
/// extra spaces.  
/// <strong class="example">Example 1:
///
/// Input: s = "the sky is blue"
/// Output: "blue is sky the"
///
/// <strong class="example">Example 2:
///
/// Input: s = "  hello world  "
/// Output: "world hello"
/// Explanation: Your reversed string should not contain leading or trailing
/// spaces.
///
/// <strong class="example">Example 3:
///
/// Input: s = "a good   example"
/// Output: "example good a"
/// Explanation: You need to reduce multiple spaces between two words to a
/// single space in the reversed string.
///
///  
/// Constraints:
///
/// 	1 <= s.length <= 10^4
/// 	s contains English letters (upper-case and lower-case), digits, and spaces
/// ' '. 	There is at least one word in s.
///
///  
/// <b data-stringify-type="bold">Follow-up: If the string data type is mutable
/// in your language, can you solve it <b data-stringify-type="bold">in-place
/// with <code data-stringify-type="code">O(1) extra space?
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-words-in-a-string/
// discuss: https://leetcode.com/problems/reverse-words-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_words(mut s: String) -> String {
        s.split(' ')
            .rev()
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn reverse_words_with_whitespace(mut s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
    // 使用chars 来解题
    pub fn reverse_words_with_point(mut s: String) -> String {
        let mut res: Vec<char> = vec![];
        let s: Vec<char> = s.trim().chars().collect::<Vec<_>>(); // 去掉首尾空格
        let mut res = String::with_capacity(s.len());
        let mut i = s.len() as i32 - 1;
        let mut j = s.len() as i32 - 1;
        while i >= 0 {
            while i >= 0 && s[i as usize] != ' ' {
                i -= 1;
            }
            res.extend(&s[i as usize..j as usize]);
            res.push(' ');
            while i >= 0 && s[i as usize] != ' ' {
                i -= 1;
            }
            j = i + 1;
        }
        res.strip_suffix(' ').unwrap().to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use regex::bytes;

    use super::*;

    #[test]
    fn test_151_1() {
        assert_eq!(
            Solution::reverse_words_with_whitespace("  hello world!  ".to_owned()),
            "world! hello".to_owned()
        );
    }

    #[test]
    fn test_151_2() {
        assert_eq!(
            Solution::reverse_words_with_point("  hello world!  ".to_owned()),
            "world! hello".to_owned()
        );
    }
    #[test]
    fn test_151_3() {
        assert_eq!(
            Solution::reverse_words(" a good   example ".to_owned()),
            "example good a".to_owned()
        );
    }
}

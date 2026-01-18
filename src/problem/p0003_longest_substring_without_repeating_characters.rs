/// [3] Longest Substring Without Repeating Characters
///
/// Given a string s, find the length of the longest <span
/// data-keyword="substring-nonempty">substring</span> without duplicate
/// characters.  
/// <strong class="example">Example 1:
///
/// Input: s = "abcabcbb"
/// Output: 3
/// Explanation: The answer is "abc", with the length of 3. Note that "bca" and
/// "cab" are also correct answers.
///
/// <strong class="example">Example 2:
///
/// Input: s = "bbbbb"
/// Output: 1
/// Explanation: The answer is "b", with the length of 1.
///
/// <strong class="example">Example 3:
///
/// Input: s = "pwwkew"
/// Output: 3
/// Explanation: The answer is "wke", with the length of 3.
/// Notice that the answer must be a substring, "pwke" is a subsequence and not
/// a substring.
///
///  
/// Constraints:
///
/// 	0 <= s.length <= 5 * 10^4
/// 	s consists of English letters, digits, symbols and spaces.
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut set = std::collections::HashSet::new();
        let mut max = 0;
        let mut left = 0;
        for (r, char) in bytes.iter().enumerate() {
            while set.contains(char) {
                set.remove(&bytes[left]);
                left += 1;
            }
            set.insert(char);
            max = max.max(r - left + 1);
        }
        max as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }
}

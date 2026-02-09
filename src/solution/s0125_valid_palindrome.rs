/// [125] Valid Palindrome
///
/// A phrase is a palindrome if, after converting all uppercase letters into
/// lowercase letters and removing all non-alphanumeric characters, it reads the
/// same forward and backward. Alphanumeric characters include letters and
/// numbers. Given a string s, return true if it is a palindrome, or false
/// otherwise.  
/// <strong class="example">Example 1:
///
/// Input: s = "A man, a plan, a canal: Panama"
/// Output: true
/// Explanation: "amanaplanacanalpanama" is a palindrome.
///
/// <strong class="example">Example 2:
///
/// Input: s = "race a car"
/// Output: false
/// Explanation: "raceacar" is not a palindrome.
///
/// <strong class="example">Example 3:
///
/// Input: s = " "
/// Output: true
/// Explanation: s is an empty string "" after removing non-alphanumeric
/// characters. Since an empty string reads the same forward and backward, it is
/// a palindrome.
///
///  
/// Constraints:
///
/// 	1 <= s.length <= 2 * 10^5
/// 	s consists only of printable ASCII characters.
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-palindrome/
// discuss: https://leetcode.com/problems/valid-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            while !s[i].is_ascii_alphanumeric() {
                i += 1;
            }
            while !s[j].is_ascii_alphanumeric() {
                j -= 1;
            }

            if s[i].eq_ignore_ascii_case(&s[j]) {
                i += 1;
                j -= 1
            } else {
                return false;
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_125() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_owned()
        ));
        assert!(!Solution::is_palindrome("race a car".to_owned()));
        assert!(!Solution::is_palindrome("0P".to_owned()));
    }
}

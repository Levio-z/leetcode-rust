use rand::rand_core::le;

/// [1456] Maximum Number of Vowels in a Substring of Given Length
///
/// Given a string s and an integer k, return the maximum number of vowel
/// letters in any substring of s with length k. Vowel letters in English are
/// 'a', 'e', 'i', 'o', and 'u'.  
/// <strong class="example">Example 1:
///
/// Input: s = "abciiidef", k = 3
/// Output: 3
/// Explanation: The substring "iii" contains 3 vowel letters.
///
/// <strong class="example">Example 2:
///
/// Input: s = "aeiou", k = 2
/// Output: 2
/// Explanation: Any substring of length 2 contains 2 vowels.
///
/// <strong class="example">Example 3:
///
/// Input: s = "leetcode", k = 3
/// Output: 2
/// Explanation: "lee", "eet" and "ode" contain 2 vowels.
///
///  
/// Constraints:
///
/// 	1 <= s.length <= 10^5
/// 	s consists of lowercase English letters.
/// 	1 <= k <= s.length
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/
// discuss: https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut count = 0;
        let mut ans = 0;
        for i in 0..k - 1 {
            if Self::is_vowel(s[i as usize]) {
                count += 1;
            }
        }
        for i in k - 1..s.len() as i32 {
            if Self::is_vowel(s[i as usize]) {
                count += 1;
            }
            if count > ans {
                ans = count;
            }
            if Self::is_vowel(s[(i - (k - 1)) as usize]) {
                count -= 1;
            }
        }
        ans
    }
    fn is_vowel(b: u8) -> bool {
        b == b'a' || b == b'e' || b == b'i' || b == b'o' || b == b'u'
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1456() {
        let s = "abciiidef".to_string();
        let k = 3;
        let ans = 3;
        assert_eq!(Solution::max_vowels(s, k), ans);
    }
}

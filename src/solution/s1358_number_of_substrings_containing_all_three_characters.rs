/// [1358] Number of Substrings Containing All Three Characters
///
/// Given a string s consisting only of characters a, b and c.
/// Return the number of substrings containing at least one occurrence of all
/// these characters a, b and c.  
/// <strong class="example">Example 1:
///
/// Input: s = "abcabc"
/// Output: 10
/// Explanation: The substrings containing at least one occurrence of the
/// characters a, b and c are "abc", "abca", "abcab", "abcabc", "bca", "bcab",
/// "bcabc", "cab", "cabc" and "abc" (again).
///
/// <strong class="example">Example 2:
///
/// Input: s = "aaacb"
/// Output: 3
/// Explanation: The substrings containing at least one occurrence of the
/// characters a, b and c are "aaacb", "aacb" and "acb".
///
/// <strong class="example">Example 3:
///
/// Input: s = "abc"
/// Output: 1
///
///  
/// Constraints:
///
/// 	3 <= s.length <= 5 x 10^4
/// 	s only consists of a, b or c characters.
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/
// discuss: https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut cnt = [0; 3];
        let mut left = 0;
        let mut ans = 0;
        for i in 0..s.len() {
            cnt[(s[i] - b'a') as usize] += 1;
            while cnt[0] > 0 && cnt[1] > 0 && cnt[2] > 0 {
                cnt[(s[left] - b'a') as usize] -= 1;
                left += 1;
            }
            ans += left as i32;
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1358() {}
}

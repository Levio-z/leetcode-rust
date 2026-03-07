/// [3090] Maximum Length Substring With Two Occurrences
///
/// Given a string s, return the maximum length of a <span
/// data-keyword="substring">substring</span> such that it contains at most two
/// occurrences of each character.  
/// <strong class="example">Example 1:
/// <div class="example-block">
/// Input: <span class="example-io">s = "bcbbbcba"</span>
/// Output: <span class="example-io">4</span>
/// Explanation:
/// The following substring has a length of 4 and contains at most two
/// occurrences of each character: "bcbb<u>bcba</u>".</div> <strong class="
/// example">Example 2: <div class="example-block">
/// Input: <span class="example-io">s = "aaaa"</span>
/// Output: <span class="example-io">2</span>
/// Explanation:
/// The following substring has a length of 2 and contains at most two
/// occurrences of each character: "<u>aa</u>aa".</div>  
/// Constraints:
///
/// 	2 <= s.length <= 100
/// 	s consists only of lowercase English letters.
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-length-substring-with-two-occurrences/
// discuss: https://leetcode.com/problems/maximum-length-substring-with-two-occurrences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut cnt = [0; 26];
        let mut left = 0;
        let mut ans = 2;
        for (i, &c) in s.iter().enumerate() {
            cnt[(c - b'a') as usize] += 1;
            while cnt[(c - b'a') as usize] > 2 {
                cnt[(s[left] - b'a') as usize] -= 1;
                left += 1;
            }
            ans = ans.max((i - left + 1) as i32);
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3090() {
        let s = "bcbbbcba".to_string();
        let ans = 4;
        assert_eq!(Solution::maximum_length_substring(s), ans);
    }
}

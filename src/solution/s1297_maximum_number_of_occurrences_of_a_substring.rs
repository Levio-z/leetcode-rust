/// [1297] Maximum Number of Occurrences of a Substring
///
/// Given a string s, return the maximum number of occurrences of any substring
/// under the following rules:
///
/// 	The number of unique characters in the substring must be less than or equal
/// to maxLetters. 	The substring size must be between minSize and maxSize
/// inclusive.
///
///  
/// <strong class="example">Example 1:
///
/// Input: s = "aababcaab", maxLetters = 2, minSize = 3, maxSize = 4
/// Output: 2
/// Explanation: Substring "aab" has 2 occurrences in the original string.
/// It satisfies the conditions, 2 unique letters and size 3 (between minSize
/// and maxSize).
///
/// <strong class="example">Example 2:
///
/// Input: s = "aaaa", maxLetters = 1, minSize = 3, maxSize = 3
/// Output: 2
/// Explanation: Substring "aaa" occur 2 times in the string. It can overlap.
///
///  
/// Constraints:
///
/// 	1 <= s.length <= 10^5
/// 	1 <= maxLetters <= 26
/// 	1 <= minSize <= maxSize <= min(26, s.length)
/// 	s consists of only lowercase English letters.
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-occurrences-of-a-substring/
// discuss: https://leetcode.com/problems/maximum-number-of-occurrences-of-a-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        let s = s.as_bytes();
        let mut char_cnt = [0; 26];
        let mut kinds = 0;
        let min_size = min_size as usize;
        let mut str_cnt = std::collections::HashMap::new();
        let mut ans = 0;
        for (i, &x) in s.iter().enumerate() {
            // 进入窗口
            let idx_char = (x - b'a') as usize;
            if char_cnt[idx_char] == 0 {
                kinds += 1;
            }
            char_cnt[idx_char] += 1;
            // 窗口条件
            if i + 1 < min_size {
                continue;
            }
            let l: usize = i - (min_size - 1);
            // 2. 统计当前窗口内的子串出现次数
            let mut cnt = 0;
            if kinds <= max_letters as usize {
                let l: usize = i - (min_size - 1);
                let mut entry = str_cnt.entry(&s[l..=i]).or_insert(0);
                *entry += 1;
                ans = ans.max(*entry);
            }
            // 离开窗口
            let idx_char = (s[l] - b'a') as usize;
            char_cnt[idx_char] -= 1;
            if char_cnt[idx_char] == 0 {
                kinds -= 1;
            }
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1297() {
        let s = "aababcaab".to_string();
        let max_letters = 2;
        let min_size = 3;
        let max_size = 4;
        let ans = 2;
        assert_eq!(Solution::max_freq(s, max_letters, min_size, max_size), ans);
    }
}

/// [541] Reverse String II
///
/// Given a string s and an integer k, reverse the first k characters for every
/// 2k characters counting from the start of the string. If there are fewer than
/// k characters left, reverse all of them. If there are less than 2k but
/// greater than or equal to k characters, then reverse the first k characters
/// and leave the other as original.  
/// <strong class="example">Example 1:
/// Input: s = "abcdefg", k = 2
/// Output: "bacdfeg"
/// <strong class="example">Example 2:
/// Input: s = "abcd", k = 2
/// Output: "bacd"
///  
/// Constraints:
///
///     1 <= s.length <= 10^4
///     s consists of only lowercase English letters.
///     1 <= k <= 10^4
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-string-ii/
// discuss: https://leetcode.com/problems/reverse-string-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_str(mut s: String, k: i32) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        for i in (0..n).step_by(2 * k as usize) {
            let end = (i + k as usize).min(n);
            chars[i..end].reverse();
        }
        chars.into_iter().collect()
    }
    pub fn reverse_str_byts(mut s: String, k: i32) -> String {
        let bytes = unsafe { s.as_bytes_mut() };
        let k: usize = k as usize;
        let n: usize = bytes.len();
        for i in (0..n).step_by(2 * k) {
            bytes[i..n.min(i + k)].reverse();
        }
        s
    }
    // 中文字符一般占 3 个字节（如 "你" → [0xE4, 0xBD, 0xA0]）

    // Emoji 占 4 个字节甚至更多

    // ASCII 仍然占 1 个字节
    fn reverse_str_pre(s: &mut [u8], i: usize, k: i32) {
        let mut i = i;
        let mut j = std::cmp::min(i + k as usize, s.len()) - 1;
        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_541() {
        assert_eq!(
            Solution::reverse_str("abcdefg".to_string(), 2),
            "bacdfeg".to_string()
        );
    }
}

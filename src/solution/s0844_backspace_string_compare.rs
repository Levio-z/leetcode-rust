/// [844] Backspace String Compare
///
/// Given two strings s and t, return true if they are equal when both are typed
/// into empty text editors. '#' means a backspace character. Note that after
/// backspacing an empty text, the text will continue empty.  
/// <strong class="example">Example 1:
///
/// Input: s = "ab#c", t = "ad#c"
/// Output: true
/// Explanation: Both s and t become "ac".
///
/// <strong class="example">Example 2:
///
/// Input: s = "ab##", t = "c#d#"
/// Output: true
/// Explanation: Both s and t become "".
///
/// <strong class="example">Example 3:
///
/// Input: s = "a#c", t = "b"
/// Output: false
/// Explanation: s becomes "c" while t becomes "b".
///
///  
/// Constraints:
///
///  <span>1 <= s.length, t.length <= 200</span>
///  <span>s and t only contain lowercase letters and '#' characters.</span>
///
///  
/// Follow up: Can you solve it in O(n) time and O(1) space?
pub struct Solution {}

// problem: https://leetcode.com/problems/backspace-string-compare/
// discuss: https://leetcode.com/problems/backspace-string-compare/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn backspace_compare1(s: String, t: String) -> bool {
        let builder = |s: String| -> String {
            let mut res = String::new();
            for c in s.chars() {
                if c == '#' {
                    // 没有直接返回None，不需要单独处理为空的情况
                    res.pop();
                } else {
                    res.push(c);
                }
            }
            res
        };
        builder(s) == builder(t)
    }
    pub fn backspace_compare(s: String, t: String) -> bool {
        let builder = |s: String| -> String {
            let mut stack = Vec::new();
            for c in s.chars() {
                if c == '#' {
                    stack.pop(); // 遇到退格键，弹出栈顶元素
                } else {
                    stack.push(c); // 普通字符，压入栈中
                }
            }
            // collect() 的目标类型是 String，而 String 实际上是通过 FromIterator<char>
            // 实现的：
            stack.into_iter().collect() // 将栈转换为字符串
        };
        builder(s) == builder(t)
    }

    pub fn backspace_compare2(s: String, t: String) -> bool {
        let (mut i, mut j) = (s.len(), t.len());
        let (s_chars, t_chars) = (s.as_bytes(), t.as_bytes());
        // 闭包：根据当前索引返回下一个有效字符索引
        let mut next_valid_index = |bytes: &[u8], mut k: usize| -> usize {
            let mut skip = 0;
            while k > 0 {
                if bytes[k - 1] == b'#' {
                    skip += 1;
                    k -= 1;
                } else if skip > 0 {
                    skip -= 1;
                    k -= 1;
                } else {
                    break;
                }
            }
            k
        };
        loop {
            i = next_valid_index(s_chars, i);
            j = next_valid_index(t_chars, j);

            if i > 0 && j > 0 {
                if s_chars[i - 1] != t_chars[j - 1] {
                    return false;
                }
            } else {
                return !(i > 0 || j > 0);
            }
            i -= 1;
            j -= 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        // 示例测试
        assert!(Solution::backspace_compare(
            "ab#c".to_string(),
            "ad#c".to_string()
        ));
        assert!(Solution::backspace_compare(
            "ab##".to_string(),
            "c#d#".to_string()
        ));

        assert!(!Solution::backspace_compare(
            "a#c".to_string(),
            "b".to_string()
        ));

        // 边界情况：空字符串
        assert!(Solution::backspace_compare("".to_string(), "".to_string()));
        assert!(Solution::backspace_compare(
            "#".to_string(),
            "#".to_string()
        ));
    }

    #[test]
    fn test_complex_cases() {
        // 连续退格
        assert!(Solution::backspace_compare(
            "abc###d".to_string(),
            "d".to_string()
        ));

        // 退格超过字符数
        assert!(Solution::backspace_compare(
            "a####b".to_string(),
            "b".to_string()
        ));

        // 相同结果不同过程
        assert!(Solution::backspace_compare(
            "a##b".to_string(),
            "b".to_string()
        ));
        assert!(Solution::backspace_compare(
            "c#b".to_string(),
            "b".to_string()
        ));
    }

    #[test]
    fn test_performance() {
        // 最大长度测试
        let long_s = "a".repeat(100) + &"#".repeat(100);
        let long_t = "b".repeat(100) + &"#".repeat(100);
        assert!(Solution::backspace_compare(long_s, long_t));
    }
}

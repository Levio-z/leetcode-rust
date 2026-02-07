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
///     0 <= s.length <= 5 * 10^4
///     s consists of English letters, digits, symbols and spaces.
///
/// # 实现思路
///
/// 无重复字符的最长子串问题可以使用滑动窗口技术高效解决。
/// 滑动窗口是一种在数组或字符串上移动的子数组或子字符串，
/// 通过维护窗口的左边界和右边界来找到满足条件的子串。
///
/// ## 实现方式概述
///
/// 1. **HashSet 实现**：
///    - 使用 HashSet 记录窗口内的字符
///    - 右指针向右移动，如果遇到重复字符，
///      左指针向右移动直到窗口内不再有重复字符
///    - 记录窗口的最大长度
///
/// 2. **数组实现**：
///    - 使用数组记录字符是否在窗口内出现过
///    - 由于字符的 ASCII 值范围有限，可以使用固定大小的数组代替 HashSet
///    - 同样使用左右指针维护窗口，记录最大长度
///
/// 3. **HashMap 优化实现**：
///    - 使用 HashMap 记录字符最后出现的位置
///    - 当遇到重复字符时，直接将左指针跳转到该字符最后出现位置的下一个位置
///    - 避免了逐步移动左指针的开销
///
/// 4. **字节优化实现**：
///    - 使用数组记录字符最近出现的位置
///    - 与 HashMap 优化实现类似，但使用数组代替 HashMap 以提高效率
///    - 适用于已知字符集的情况（如 ASCII 字符集）
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    /// 使用 HashSet 实现的滑动窗口解法
    ///
    /// ## 核心思路
    /// - 维护一个 HashSet 记录当前窗口内的字符
    /// - 右指针 r 向右移动，扩展窗口
    /// - 如果当前字符已存在于窗口中，左指针 l 向右移动直到窗口中不再包含该字符
    /// - 记录窗口的最大长度
    ///
    /// ## 复杂度分析
    /// - 时间复杂度：O(n)，每个字符最多被访问两次
    /// - 空间复杂度：O(min(m, n))，m 是字符集大小
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

    /// 使用数组实现的滑动窗口解法
    ///
    /// ## 核心思路
    /// - 使用固定大小的数组代替 HashSet，提高访问效率
    /// - 数组索引对应字符的 ASCII 值，数组值表示字符是否在窗口中
    /// - 同样使用左右指针维护窗口，记录最大长度
    ///
    /// ## 复杂度分析
    /// - 时间复杂度：O(n)
    /// - 空间复杂度：O(1)，数组大小固定为 128
    pub fn length_of_longest_substring2(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        let mut l = 0;
        let mut cnt = [false; 128]; // 使用数组代替HashSet，提高效率

        for (r, &c) in s.iter().enumerate() {
            let c = c as usize;
            // 如果窗口内已经包含 c，缩小窗口直到不包含 c
            while cnt[c] {
                cnt[s[l] as usize] = false;
                l += 1;
            }
            cnt[c] = true; // 将 c 加入窗口
            ans = ans.max(r - l + 1); // 更新最大窗口长度
        }

        ans as _
    }

    /// 使用 HashMap 优化的滑动窗口解法
    ///
    /// ## 核心思路
    /// - 使用 HashMap 记录字符最后出现的位置
    /// - 当遇到重复字符时，直接将左指针跳转到该字符最后出现位置的下一个位置
    /// - 避免了逐步移动左指针的开销
    ///
    /// ## 复杂度分析
    /// - 时间复杂度：O(n)
    /// - 空间复杂度：O(min(m, n))
    pub fn length_of_longest_substring3(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut cnt = std::collections::HashMap::new();
        let mut ans = 0;
        let mut l = 0;

        for (r, &char) in bytes.iter().enumerate() {
            // 如果字符已存在，直接将左指针跳转到该字符最后出现位置的下一个位置
            if let Some(&i) = cnt.get(&char) {
                l = l.max(i + 1);
            }
            // 更新字符最后出现的位置
            cnt.insert(char, r);
            // 更新最大窗口长度
            ans = ans.max(r - l + 1);
        }

        ans as i32
    }

    /// 使用字节数组优化的滑动窗口解法
    ///
    /// ## 核心思路
    /// - 使用数组记录字符最近出现的位置
    /// - 与 HashMap 优化实现类似，但使用数组代替 HashMap 以提高效率
    /// - 适用于已知字符集的情况（如 ASCII 字符集）
    ///
    /// ## 复杂度分析
    /// - 时间复杂度：O(n)
    /// - 空间复杂度：O(1)，数组大小固定为 128
    pub fn length_of_longest_substring_byte_opt(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut last = [-1; 128]; // 记录ASCII字符最近出现的位置
        let mut l = 0i32;
        let mut ans = 0i32;

        for (r, &c) in bytes.iter().enumerate() {
            let idx = c as usize;
            // 如果该字符之前出现过，左指针直接跳转
            if last[idx] >= l {
                l = last[idx] + 1;
            }
            // 更新字符最近出现的位置
            last[idx] = r as i32;
            // 更新最大窗口长度
            ans = ans.max(r as i32 - l + 1);
        }

        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        // 测试示例1
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        // 测试示例2
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        // 测试示例3
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        // 测试空字符串
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
        // 测试所有字符都不同的情况
        assert_eq!(
            Solution::length_of_longest_substring("abcdefg".to_string()),
            7
        );
        // 测试包含空格和特殊字符的情况
        assert_eq!(
            Solution::length_of_longest_substring("a b c a b c".to_string()),
            3
        );
    }

    #[test]
    fn test_3_array() {
        // 测试数组实现
        assert_eq!(
            Solution::length_of_longest_substring2("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring2("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring2("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn test_3_hashmap() {
        // 测试HashMap优化实现
        assert_eq!(
            Solution::length_of_longest_substring3("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring3("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring3("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn test_3_byte_opt() {
        // 测试字节优化实现
        assert_eq!(
            Solution::length_of_longest_substring_byte_opt("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring_byte_opt("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring_byte_opt("pwwkew".to_string()),
            3
        );
    }
}

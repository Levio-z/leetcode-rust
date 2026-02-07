use rand::rand_core::le;

/// [344] Reverse String
///
/// Write a function that reverses a string. The input string is given as an
/// array of characters s. You must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with O(1) extra memory.
///  
/// <strong class="example">Example 1:
/// Input: s = ["h","e","l","l","o"]
/// Output: ["o","l","l","e","h"]
/// <strong class="example">Example 2:
/// Input: s = ["H","a","n","n","a","h"]
/// Output: ["h","a","n","n","a","H"]
///  
/// Constraints:
///
///     1 <= s.length <= 10^5
///     s[i] is a <a href="https://en.wikipedia.org/wiki/ASCII#Printable_characters" target="_blank">printable ascii character</a>.
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-string/
// discuss: https://leetcode.com/problems/reverse-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_string(s: &mut [char]) {
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            // panic when out of bounds.
            s.swap(i, j);
            // 相当于手动let temp = s[i];s[i] = s[j];s[j] = temp;
            // 切片上更好的方法 s.reverse();
            i += 1;
            j -= 1;
        }
        s.reverse();
    }
    /// 收集器,所有权转移,迭代器风格
    pub fn reverse_string1(mut s: &mut Vec<char>) {
        *s = s.iter().rev().copied().collect::<Vec<_>>();
    }
    /// 元组解构
    pub fn reverse_string2(mut s: &mut [char]) {
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            (s[i], s[j]) = (s[j], s[i]);
            i += 1;
            j -= 1;
        }
    }
    /// 递归
    pub fn reverse_string3(mut s: &mut [char]) {
        Self::reverse_recursive(s, 0, s.len() - 1);
    }
    fn reverse_recursive(s: &mut [char], left: usize, right: usize) {
        if left >= right {
            return;
        }
        s.swap(left, right);
        Self::reverse_recursive(s, left + 1, right - 1);
    }
    /// 分而治之
    pub fn reverse_string4(mut s: &mut [char]) {
        if s.len() <= 1 {
            return;
        }
        // mid 始终是左半部分的长度
        // mid - 1 始终是左半部分的最后一个元素
        let mid = s.len() / 2;
        let (left, right) = s.split_at_mut(mid);
        // more idiomatic
        for (l, r) in left.iter_mut().zip(right.iter().rev()) {
            *l = *r;
        }
    }
    /// str上的一些方法也可以研究一下
    fn reverse_string_ascii(s: &mut str) {
        unsafe {
            // unsafe function
            let bytes = s.as_bytes_mut();
            bytes.reverse();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_344() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }
}

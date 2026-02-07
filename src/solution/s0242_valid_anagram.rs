/// [242] Valid Anagram
///
/// Given two strings s and t, return true if t is an <span
/// data-keyword="anagram">anagram</span> of s, and false otherwise.  
/// <strong class="example">Example 1:
/// <div class="example-block">
/// Input: <span class="example-io">s = "anagram", t = "nagaram"</span>
/// Output: <span class="example-io">true</span>
/// </div>
/// <strong class="example">Example 2:
/// <div class="example-block">
/// Input: <span class="example-io">s = "rat", t = "car"</span>
/// Output: <span class="example-io">false</span>
/// </div>
///  
/// Constraints:
///
///     1 <= s.length, t.length <= 5 * 10^4
///     s and t consist of lowercase English letters.
///
///  
/// Follow up: What if the inputs contain Unicode characters? How would you
/// adapt your solution to such a case?
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-anagram/
// discuss: https://leetcode.com/problems/valid-anagram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut byte = [0; 26];
        s.bytes().for_each(|b| byte[(b - b'a') as usize] += 1);
        t.bytes().for_each(|b| byte[(b - b'a') as usize] -= 1);
        // all 方法会返回一个布尔值，表示迭代器中的所有元素是否都满足给定的条件
        // all 是消耗性方法，调用后迭代器将被消耗，无法再次使用
        // 消耗迭代器（推进游标）
        byte.iter().all(|x| *x == 0)
    }
    pub fn is_anagram2(s: String, t: String) -> bool {
        let mut byte = std::collections::HashMap::new();
        // or_insert 返回的是值的可变引用 &'a mut V
        s.bytes().for_each(|b| *byte.entry(b).or_insert(0) += 1);
        t.bytes().for_each(|b| *byte.entry(b).or_insert(0) -= 1);
        byte.iter().all(|(_, x)| *x == 0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_242() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    }
    #[test]
    fn test_242_2() {
        assert!(Solution::is_anagram2(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
        assert!(!Solution::is_anagram2("rat".to_string(), "car".to_string()));
    }
}

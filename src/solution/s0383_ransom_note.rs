/// [383] Ransom Note
///
/// Given two strings ransomNote and magazine, return true if ransomNote can be
/// constructed by using the letters from magazine and false otherwise.
/// Each letter in magazine can only be used once in ransomNote.
///  
/// <strong class="example">Example 1:
/// Input: ransomNote = "a", magazine = "b"
/// Output: false
/// <strong class="example">Example 2:
/// Input: ransomNote = "aa", magazine = "ab"
/// Output: false
/// <strong class="example">Example 3:
/// Input: ransomNote = "aa", magazine = "aab"
/// Output: true
///  
/// Constraints:
///
///     1 <= ransomNote.length, magazine.length <= 10^5
///     ransomNote and magazine consist of lowercase English letters.
pub struct Solution {}

// problem: https://leetcode.com/problems/ransom-note/
// discuss: https://leetcode.com/problems/ransom-note/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut cnt = [0; 26];
        ransom_note
            .bytes()
            // 可以获得外部的可变引用
            .for_each(|b| cnt[(b - b'a') as usize] += 1);
        magazine.bytes().for_each(|b| cnt[(b - b'a') as usize] -= 1);
        cnt.iter().all(|&x| x <= 0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_383() {}
}

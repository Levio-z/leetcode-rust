/// [3258] Count Substrings That Satisfy K-Constraint I
///
/// You are given a binary string s and an integer k.
/// A binary string satisfies the k-constraint if either of the following
/// conditions holds:
///
/// 	The number of 0's in the string is at most k.
/// 	The number of 1's in the string is at most k.
///
/// Return an integer denoting the number of <span
/// data-keyword="substring-nonempty">substrings</span> of s that satisfy the
/// k-constraint.  
/// <strong class="example">Example 1:
/// <div class="example-block">
/// Input: <span class="example-io">s = "10101", k = 1</span>
/// Output: <span class="example-io">12</span>
/// Explanation:
/// Every substring of s except the substrings "1010", "10101", and "0101"
/// satisfies the k-constraint. </div>
/// <strong class="example">Example 2:
/// <div class="example-block">
/// Input: <span class="example-io">s = "1010101", k = 2</span>
/// Output: <span class="example-io">25</span>
/// Explanation:
/// Every substring of s except the substrings with a length greater than 5
/// satisfies the k-constraint. </div>
/// <strong class="example">Example 3:
/// <div class="example-block">
/// Input: <span class="example-io">s = "11111", k = 1</span>
/// Output: <span class="example-io">15</span>
/// Explanation:
/// All substrings of s satisfy the k-constraint.
/// </div>
///  
/// Constraints:
///
/// 	1 <= s.length <= 50
/// 	1 <= k <= s.length
/// 	s[i] is either '0' or '1'.
pub struct Solution {}

// problem: https://leetcode.com/problems/count-substrings-that-satisfy-k-constraint-i/
// discuss: https://leetcode.com/problems/count-substrings-that-satisfy-k-constraint-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut cnt = vec![0; 2];
        let mut ans = 0;
        let k = k as usize;
        let mut left = 0;
        for (i, &x) in s.iter().enumerate() {
            cnt[x as usize - '0' as usize] += 1;
            while cnt[0] > k && cnt[1] > k {
                cnt[s[left] as usize - '0' as usize] -= 1;
                left += 1;
            }
            ans += i - left + 1;
        }
        ans as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3258() {
        assert_eq!(
            Solution::count_k_constraint_substrings("10101".to_string(), 1),
            12
        );
    }
}

/// [611] Valid Triangle Number
///
/// Given an integer array nums, return the number of triplets chosen from the
/// array that can make triangles if we take them as side lengths of a triangle.
///
/// <strong class="example">Example 1:
///
/// Input: nums = [2,2,3,4]
/// Output: 3
/// Explanation: Valid combinations are:
/// 2,3,4 (using the first 2)
/// 2,3,4 (using the second 2)
/// 2,2,3
///
/// <strong class="example">Example 2:
///
/// Input: nums = [4,2,3,4]
/// Output: 4
///
///  
/// Constraints:
///
///     1 <= nums.length <= 1000
///     0 <= nums[i] <= 1000
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-triangle-number/
// discuss: https://leetcode.com/problems/valid-triangle-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = 0;
        for i in 0..n.saturating_sub(2) {
            let a = nums[i];
            if a == 0 {
                continue;
            }
            let mut j = i + 1;
            for k in i + 2..n {
                while a + nums[j] <= nums[k] {
                    j += 1;
                }
                ans += k - j;
            }
        }
        ans as _
    }
    pub fn triangle_number2(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ans = 0;
        let n = nums.len();
        for k in 2..n {
            let c = nums[k];
            let mut i = 0;
            let mut j = k - 1;
            while i < j {
                if nums[i] + nums[j] > c {
                    ans += j - i;
                    i += 1;
                } else {
                    i += 1;
                }
            }
        }
        ans as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_611() {
        let nums = vec![2, 2, 3, 4];
        assert_eq!(Solution::triangle_number(nums), 3);
    }
    #[test]
    fn test_611_2() {
        let nums = vec![2, 3, 4, 4];
        assert_eq!(Solution::triangle_number2(nums), 4);
    }
}

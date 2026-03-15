/// [2439] Minimize Maximum of Array
///
/// You are given a 0-indexed array nums comprising of n non-negative integers.
/// In one operation, you must:
///
/// 	Choose an integer i such that 1 <= i < n and nums[i] > 0.
/// 	Decrease nums[i] by 1.
/// 	Increase nums[i - 1] by 1.
///
/// Return the minimum possible value of the maximum integer of nums after
/// performing any number of operations.  
/// <strong class="example">Example 1:
///
/// Input: nums = [3,7,1,6]
/// Output: 5
/// Explanation:
/// One set of optimal operations is as follows:
/// 1. Choose i = 1, and nums becomes [4,6,1,6].
/// 2. Choose i = 3, and nums becomes [4,6,2,5].
/// 3. Choose i = 1, and nums becomes [5,5,2,5].
/// The maximum integer of nums is 5. It can be shown that the maximum number
/// cannot be less than 5. Therefore, we return 5.
///
/// <strong class="example">Example 2:
///
/// Input: nums = [10,1]
/// Output: 10
/// Explanation:
/// It is optimal to leave nums as is, and since 10 is the maximum value, we
/// return 10.
///
///  
/// Constraints:
///
/// 	n == nums.length
/// 	2 <= n <= 10^5
/// 	0 <= nums[i] <= 10^9
pub struct Solution {}

// problem: https://leetcode.com/problems/minimize-maximum-of-array/
// discuss: https://leetcode.com/problems/minimize-maximum-of-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimize_array_value(mut nums: Vec<i32>) -> i32 {
        fn check(nums: &[i32], limit: i64) -> bool {
            let mut extra = 0;
            for &i in nums.iter().rev() {
                let i = i as i64;
                let new_num = i + extra;
                extra = (new_num - limit).max(0);
            }
            extra <= 0
        }
        let mut r = *nums.iter().max().unwrap();
        let mut l = nums[0] - 1;
        while l + 1 < r {
            let mid = l + (r - l) / 2;
            if (check(&nums, mid as i64)) {
                r = mid;
            } else {
                l = mid;
            }
        }
        r
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2439() {}
}

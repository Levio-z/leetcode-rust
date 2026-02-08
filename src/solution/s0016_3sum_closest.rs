use core::num;

/// [16] 3Sum Closest
///
/// Given an integer array nums of length n and an integer target, find three
/// integers at distinct indices in nums such that the sum is closest to target.
/// Return the sum of the three integers.
/// You may assume that each input would have exactly one solution.
///  
/// <strong class="example">Example 1:
///
/// Input: nums = [-1,2,1,-4], target = 1
/// Output: 2
/// Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
///
/// <strong class="example">Example 2:
///
/// Input: nums = [0,0,0], target = 1
/// Output: 0
/// Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
///
///  
/// Constraints:
///
///     3 <= nums.length <= 500
///     -1000 <= nums[i] <= 1000
///     -10^4 <= target <= 10^4
pub struct Solution {}

// problem: https://leetcode.com/problems/3sum-closest/
// discuss: https://leetcode.com/problems/3sum-closest/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = 0;
        let mut diff = i32::MAX;
        for i in 0..n - 2 {
            let x = nums[i];
            // 跳过重复元素
            if i > 0 && x == nums[i - 1] {
                continue;
            }
            let min_value = x + nums[i + 1] + nums[i + 2];
            if min_value > target {
                Self::update_ans(x, &mut ans, &mut diff, min_value, &nums, target);
                break;
            }
            let max_value = x + nums[n - 2] + nums[n - 1];
            if max_value < target {
                Self::update_ans(x, &mut ans, &mut diff, max_value, &nums, target);
                continue;
            }
            let mut j = i + 1;
            let mut k = n - 1;
            while (j < k) {
                let sum = x + nums[j] + nums[k];
                if sum == target {
                    return sum;
                }
                Self::update_ans(x, &mut ans, &mut diff, sum, &nums, target);
                if sum < target {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        ans
    }
    fn update_ans(x: i32, ans: &mut i32, diff: &mut i32, sum: i32, nums: &[i32], target: i32) {
        let cur_diff = (sum - target).abs();
        if cur_diff < *diff {
            *diff = cur_diff;
            *ans = sum;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_16() {
        let nums = vec![-1, 2, 1, -4];
        let target = 1;
        let ans = 2;
        assert_eq!(Solution::three_sum_closest(nums, target), ans);
    }
}

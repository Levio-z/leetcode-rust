use rand::rand_core::le;

/// [2962] Count Subarrays Where Max Element Appears at Least K Times
///
/// You are given an integer array nums and a positive integer k.
/// Return the number of subarrays where the maximum element of nums appears at
/// least k times in that subarray. A subarray is a contiguous sequence of
/// elements within an array.  
/// <strong class="example">Example 1:
///
/// Input: nums = [1,3,2,3,3], k = 2
/// Output: 6
/// Explanation: The subarrays that contain the element 3 at least 2 times are:
/// [1,3,2,3], [1,3,2,3,3], [3,2,3], [3,2,3,3], [2,3,3] and [3,3].
///
/// <strong class="example">Example 2:
///
/// Input: nums = [1,4,2,1], k = 3
/// Output: 0
/// Explanation: No subarray contains the element 4 at least 3 times.
///
///  
/// Constraints:
///
/// 	1 <= nums.length <= 10^5
/// 	1 <= nums[i] <= 10^6
/// 	1 <= k <= 10^5
pub struct Solution {}

// problem: https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/
// discuss: https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max = *nums.iter().max().unwrap();
        let mut cnt = 0;
        let mut left = 0;
        let mut ans = 0;
        for i in 0..nums.len() {
            if nums[i] == max {
                cnt += 1;
            }
            while cnt >= k {
                if nums[left] == max {
                    cnt -= 1;
                }
                left += 1;
            }
            ans += left;
        }
        ans as i64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2962() {
        let nums = vec![1, 3, 2, 3, 3];
        let k = 2;
        let ans = 6;
        assert_eq!(Solution::count_subarrays(nums, k), ans);
    }
}

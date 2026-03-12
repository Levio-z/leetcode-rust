/// [2563] Count the Number of Fair Pairs
///
/// Given a 0-indexed integer array nums of size n and two integers lower and
/// upper, return the number of fair pairs. A pair (i, j) is fair if:
///
/// 	0 <= i < j < n, and
/// 	lower <= nums[i] + nums[j] <= upper
///
///  
/// <strong class="example">Example 1:
///
/// Input: nums = [0,1,7,4,4,5], lower = 3, upper = 6
/// Output: 6
/// Explanation: There are 6 fair pairs: (0,3), (0,4), (0,5), (1,3), (1,4), and
/// (1,5).
///
/// <strong class="example">Example 2:
///
/// Input: nums = [1,7,9,2,5], lower = 11, upper = 11
/// Output: 1
/// Explanation: There is a single fair pair: (2,3).
///
///  
/// Constraints:
///
/// 	1 <= nums.length <= 10^5
/// 	nums.length == n
/// 	<font face="monospace">-10^9</font> <= nums[i] <= 10^9
/// 	<font face="monospace">-10^9 <= lower <= upper <= 10^9</font>
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-number-of-fair-pairs/
// discuss: https://leetcode.com/problems/count-the-number-of-fair-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        let mut ans = 0;
        for j in 0..nums.len() {
            let l = nums[..j].partition_point(|&x| x < lower - nums[j]);
            let r = nums[..j].partition_point(|&x| x <= upper - nums[j]);
            ans += r - l;
        }
        ans as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2563_example1() {
        let nums = vec![0, 1, 7, 4, 4, 5];
        let lower = 3;
        let upper = 6;
        assert_eq!(Solution::count_fair_pairs(nums, lower, upper), 6);
    }

    #[test]
    fn test_2563_example2() {
        let nums = vec![1, 7, 9, 2, 5];
        let lower = 11;
        let upper = 11;
        assert_eq!(Solution::count_fair_pairs(nums, lower, upper), 1);
    }

    #[test]
    fn test_2563_empty_range() {
        let nums = vec![1, 2, 3];
        let lower = 10;
        let upper = 20;
        assert_eq!(Solution::count_fair_pairs(nums, lower, upper), 0);
    }

    #[test]
    fn test_2563_negative_numbers() {
        let nums = vec![-3, -1, 0, 2, 4];
        let lower = -2;
        let upper = 2;
        assert_eq!(Solution::count_fair_pairs(nums, lower, upper), 4);
    }
}

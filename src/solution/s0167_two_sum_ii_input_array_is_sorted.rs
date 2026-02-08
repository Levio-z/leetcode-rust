use std::vec;

/// [167] Two Sum II - Input Array Is Sorted
///
/// Given a 1-indexed array of integers numbers that is already sorted in
/// non-decreasing order, find two numbers such that they add up to a specific
/// target number. Let these two numbers be numbers[index1] and numbers[index2]
/// where 1 <= index1 < index2 <= numbers.length. Return the indices of the two
/// numbers, index1 and index2, added by one as an integer array [index1,
/// index2] of length 2. The tests are generated such that there is exactly one
/// solution. You may not use the same element twice. Your solution must use
/// only constant extra space.  
/// <strong class="example">Example 1:
///
/// Input: numbers = [<u>2</u>,<u>7</u>,11,15], target = 9
/// Output: [1,2]
/// Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We
/// return [1, 2].
///
/// <strong class="example">Example 2:
///
/// Input: numbers = [<u>2</u>,3,<u>4</u>], target = 6
/// Output: [1,3]
/// Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We
/// return [1, 3].
///
/// <strong class="example">Example 3:
///
/// Input: numbers = [<u>-1</u>,<u>0</u>], target = -1
/// Output: [1,2]
/// Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We
/// return [1, 2].
///
///  
/// Constraints:
///
///     2 <= numbers.length <= 3 * 10^4
///     -1000 <= numbers[i] <= 1000
///     numbers is sorted in non-decreasing order.
///     -1000 <= target <= 1000
///     The tests are generated such that there is exactly one solution.
pub struct Solution {}

// problem: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
// discuss: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while (l < r) {
            let sum = nums[l] + nums[r];
            if sum == target {
                return vec![l as i32 + 1, r as i32 + 1];
            } else if sum < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
        vec![]
    }
    pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = nums.len() - 1;
        loop {
            let sum = nums[l] + nums[r];
            if sum == target {
                return vec![l as i32 + 1, r as i32 + 1];
            } else if sum < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_167() {
        let nums = vec![1, 2, 2, 7, 11, 15, 20, 40];
        let target = 31;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![5, 7]);
    }
    #[test]
    fn test_167_2() {
        let nums = vec![1, 2, 2, 7, 11, 15, 20, 40];
        let target = 31;
        let result = Solution::two_sum_2(nums, target);
        assert_eq!(result, vec![5, 7]);
    }
}

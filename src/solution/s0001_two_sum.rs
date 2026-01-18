/// [1] Two Sum
///
/// Given an array of integers nums and an integer target, return indices of the
/// two numbers such that they add up to target. You may assume that each input
/// would have exactly one solution, and you may not use the same element twice.
/// You can return the answer in any order.
///  
/// <strong class="example">Example 1:
///
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1]
/// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
///
/// <strong class="example">Example 2:
///
/// Input: nums = [3,2,4], target = 6
/// Output: [1,2]
///
/// <strong class="example">Example 3:
///
/// Input: nums = [3,3], target = 6
/// Output: [0,1]
///
///  
/// Constraints:
///
/// 2 <= nums.length <= 10^4
/// -10^9 <= nums[i] <= 10^9
/// -10^9 <= target <= 10^9
/// Only one valid answer exists.
///
///  
/// Follow-up: Can you come up with an algorithm that is less than O(n^2)<font
/// face="monospace"> </font>time complexity?
pub struct Solution {}

// problem: https://leetcode.com/problems/two-sum/
// discuss: https://leetcode.com/problems/two-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
// 每次遍历都是求当前元素和前面的元素有没有解
// 状态不需要遍历所有状态，只要条件满足就退出
// 如果使用遍历的方式求当前元素和前面元素有没有解，时间复杂度是O(n^2)
// 暴力解法：
// for i in 0..n {
//     for j in 0..i {
//         if nums[i] + nums[j] == target {
//             return [i, j];
//         }
//     }
// }
// 内层时间复杂度为O(n)，外层时间复杂度为O(n)，总时间复杂度为O(n^2)
// 使用哈希表存储前面遍历过的元素，内层时间复杂度为O(1)，总时间复杂度为O(n)
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 创建hash表
        let mut idx = HashMap::with_capacity(nums.len());
        for (i, &x) in nums.iter().enumerate() {
            match idx.get(&(target - x)) {
                None => idx.insert(x, i),
                Some(&j) => return vec![i as i32, j as i32],
            };
        }
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 0], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![2, 1], Solution::two_sum(vec![3, 2, 4], 6));
    }
}

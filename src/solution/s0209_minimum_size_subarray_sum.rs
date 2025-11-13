/// [209] Minimum Size Subarray Sum
///
/// Given an array of positive integers nums and a positive integer target,
/// return the minimal length of a <span
/// data-keyword="subarray-nonempty">subarray</span> whose sum is greater than
/// or equal to target. If there is no such subarray, return 0 instead.  
/// <strong class="example">Example 1:
///
/// Input: target = 7, nums = [2,3,1,2,4,3]
/// Output: 2
/// Explanation: The subarray [4,3] has the minimal length under the problem
/// constraint.
///
/// <strong class="example">Example 2:
///
/// Input: target = 4, nums = [1,4,4]
/// Output: 1
///
/// <strong class="example">Example 3:
///
/// Input: target = 11, nums = [1,1,1,1,1,1,1,1]
/// Output: 0
///
///  
/// Constraints:
///
/// 	1 <= target <= 10^9
/// 	1 <= nums.length <= 10^5
/// 	1 <= nums[i] <= 10^4
///
///  
/// Follow up: If you have figured out the O(n) solution, try coding another
/// solution of which the time complexity is O(n log(n)).
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-size-subarray-sum/
// discuss: https://leetcode.com/problems/minimum-size-subarray-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_sub_array_len1(target: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut ans = nums.len() + 1;
        let mut num = 0;
        let mut left = 0;
        for (i, &x) in nums.iter().enumerate() {
            sum += x;
            num += 1;
            while (sum >= target) {
                ans = num.min(ans);
                sum -= nums[left];
                left += 1;
                num -= 1;
            }
        }
        if ans > nums.len() { 0 } else { ans as _ }
    }

    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = n + 1;
        let mut sum = 0; // 子数组元素和
        let mut left = 0; //子数组右端点
        for (right, &x) in nums.iter().enumerate() {
            sum += x;
            while sum >= target {
                ans = ans.min(right - left + 1);
                sum -= nums[left];
                left += 1; //左端点右移
            }
        }
        if ans <= n { ans as i32 } else { 0 }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        // 示例测试
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }

    #[test]
    fn test_edge_cases() {
        // 单个元素满足条件
        assert_eq!(Solution::min_sub_array_len(5, vec![5]), 1);

        // 单个元素不满足条件
        assert_eq!(Solution::min_sub_array_len(6, vec![5]), 0);

        // 需要整个数组才能满足条件
        assert_eq!(Solution::min_sub_array_len(15, vec![1, 2, 3, 4, 5]), 5);

        // 数组开头就满足条件
        assert_eq!(Solution::min_sub_array_len(3, vec![3, 1, 1, 1]), 1);

        // 数组末尾满足条件
        assert_eq!(Solution::min_sub_array_len(3, vec![1, 1, 1, 3]), 1);

        // 重复元素
        assert_eq!(Solution::min_sub_array_len(8, vec![2, 2, 2, 2, 2]), 4);
    }

    #[test]
    fn test_large_input() {
        // 大规模数据测试
        let large_nums = vec![1; 100000];
        assert_eq!(Solution::min_sub_array_len(100000, large_nums), 100000);
    }
}

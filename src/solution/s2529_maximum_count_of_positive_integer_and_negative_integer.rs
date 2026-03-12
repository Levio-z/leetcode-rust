/// [2529] Maximum Count of Positive Integer and Negative Integer
///
/// Given an array nums sorted in non-decreasing order, return the maximum
/// between the number of positive integers and the number of negative integers.
///
/// 	In other words, if the number of positive integers in nums is pos and the
/// number of negative integers is neg, then return the maximum of pos and neg.
///
/// Note that 0 is neither positive nor negative.
///  
/// <strong class="example">Example 1:
///
/// Input: nums = [-2,-1,-1,1,2,3]
/// Output: 3
/// Explanation: There are 3 positive integers and 3 negative integers. The
/// maximum count among them is 3.
///
/// <strong class="example">Example 2:
///
/// Input: nums = [-3,-2,-1,0,0,1,2]
/// Output: 3
/// Explanation: There are 2 positive integers and 3 negative integers. The
/// maximum count among them is 3.
///
/// <strong class="example">Example 3:
///
/// Input: nums = [5,20,66,1314]
/// Output: 4
/// Explanation: There are 4 positive integers and 0 negative integers. The
/// maximum count among them is 4.
///
///  
/// Constraints:
///
/// 	1 <= nums.length <= 2000
/// 	-2000 <= nums[i] <= 2000
/// 	nums is sorted in a non-decreasing order.
///
///  
/// Follow up: Can you solve the problem in O(log(n)) time complexity?
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/
// discuss: https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut l = 0;
        let mut r = n;
        while l < r {
            let mid = l + (r - l) / 2;
            if nums[mid] >= 0 {
                r = mid;
            } else {
                l += 1;
            }
        }
        let mut m = l;
        while m < n && nums[m] == 0 {
            m += 1;
        }
        l.max(n - l) as _
    }

    pub fn maximum_count_1(nums: Vec<i32>) -> i32 {
        // 计算出来的结果就是<0的数量
        let neg = nums.partition_point(|&x| x < 0);
        // 计算出来的结果就是<=0的数量
        let pos = nums.len() - nums.partition_point(|&x| x <= 0);
        neg.max(pos) as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2529() {
        let vec = vec![-3, -2, 1, 2, 3];
        assert_eq!(Solution::maximum_count(vec), 3);
        println!()
        // 测试
    }
}

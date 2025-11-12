/// [977] Squares of a Sorted Array
///
/// Given an integer array nums sorted in non-decreasing order, return an array
/// of the squares of each number sorted in non-decreasing order.  
/// <strong class="example">Example 1:
///
/// Input: nums = [-4,-1,0,3,10]
/// Output: [0,1,9,16,100]
/// Explanation: After squaring, the array becomes [16,1,0,9,100].
/// After sorting, it becomes [0,1,9,16,100].
///
/// <strong class="example">Example 2:
///
/// Input: nums = [-7,-3,2,3,11]
/// Output: [4,9,9,49,121]
///
///  
/// Constraints:
///
/// <span>1 <= nums.length <= </span>10^4
/// -10^4 <= nums[i] <= 10^4
/// nums is sorted in non-decreasing order.
///
///  
/// Follow up: Squaring each element and sorting the new array is very trivial,
/// could you find an O(n) solution using a different approach?
pub struct Solution {}

// problem: https://leetcode.com/problems/squares-of-a-sorted-array/
// discuss: https://leetcode.com/problems/squares-of-a-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut n = nums.len();
        let mut ans = vec![0; n];
        let mut l = 0;
        let mut r = n;
        while l < r {
            if -nums[l] > nums[r - 1] {
                ans[n - 1] = nums[l] * nums[l];
                l += 1;
            } else {
                ans[n - 1] = nums[r - 1] * nums[r - 1];
                r -= 1;
            }
            n -= 1;
        }
        ans
    }
    pub fn sorted_squares1(nums: Vec<i32>) -> Vec<i32> {
        // Vec<T>::iter() 方法返回一个 不可变引用迭代器：
        // 也就是说，它产生的序列元素类型是 &T。
        let mut squared: Vec<i32> = nums.iter().map(|&x| x * x).collect();
        squared.sort();
        squared
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_977() {
        // 示例1: 包含负数、零和正数的数组
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );

        // 示例2: 包含负数和正数的数组
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );

        // 边界情况1: 所有元素都是正数
        assert_eq!(
            Solution::sorted_squares(vec![1, 2, 3, 4, 5]),
            vec![1, 4, 9, 16, 25]
        );

        // 边界情况2: 所有元素都是负数
        assert_eq!(
            Solution::sorted_squares(vec![-5, -4, -3, -2, -1]),
            vec![1, 4, 9, 16, 25]
        );

        // 边界情况3: 单个元素
        assert_eq!(Solution::sorted_squares(vec![0]), vec![0]);
        assert_eq!(Solution::sorted_squares(vec![5]), vec![25]);
        assert_eq!(Solution::sorted_squares(vec![-5]), vec![25]);

        // 边界情况4: 两个元素
        assert_eq!(Solution::sorted_squares(vec![-3, 2]), vec![4, 9]);
        assert_eq!(Solution::sorted_squares(vec![-2, -1]), vec![1, 4]);
        assert_eq!(Solution::sorted_squares(vec![1, 2]), vec![1, 4]);

        // 边界情况5: 包含重复元素的数组
        assert_eq!(
            Solution::sorted_squares(vec![-2, -2, 1, 1, 3]),
            vec![1, 1, 4, 4, 9]
        );

        // 边界情况6: 最小长度数组
        assert_eq!(Solution::sorted_squares(vec![1]), vec![1]);
    }
}

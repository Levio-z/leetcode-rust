/// [2653] Sliding Subarray Beauty
///
/// Given an integer array nums containing n integers, find the beauty of each
/// subarray of size k. The beauty of a subarray is the x^th smallest integer in
/// the subarray if it is negative, or 0 if there are fewer than x negative
/// integers. Return an integer array containing n - k + 1 integers, which
/// denote the beauty of the subarrays in order from the first index in the
/// array.
///
///
/// 	A subarray is a contiguous non-empty sequence of elements within an array.
///
///
///  
/// <strong class="example">Example 1:
///
/// Input: nums = [1,-1,-3,-2,3], k = 3, x = 2
/// Output: [-1,-2,-2]
/// Explanation: There are 3 subarrays with size k = 3.
/// The first subarray is [1, -1, -3] and the 2^nd smallest negative integer is
/// -1. The second subarray is [-1, -3, -2] and the 2^nd smallest negative
/// integer is -2. The third subarray is [-3, -2, 3] and the 2^nd smallest
/// negative integer is -2. <strong class="example">Example 2:
///
/// Input: nums = [-1,-2,-3,-4,-5], k = 2, x = 2
/// Output: [-1,-2,-3,-4]
/// Explanation: There are 4 subarrays with size k = 2.
/// For [-1, -2], the 2^nd smallest negative integer is -1.
/// For [-2, -3], the 2^nd smallest negative integer is -2.
/// For [-3, -4], the 2^nd smallest negative integer is -3.
/// For [-4, -5], the 2^nd smallest negative integer is -4.
/// <strong class="example">Example 3:
///
/// Input: nums = [-3,1,2,-3,0,-3], k = 2, x = 1
/// Output: [-3,0,-3,-3,-3]
/// Explanation: There are 5 subarrays with size k = 2.
/// For [-3, 1], the 1^st smallest negative integer is -3.
/// For [1, 2], there is no negative integer so the beauty is 0.
/// For [2, -3], the 1^st smallest negative integer is -3.
/// For [-3, 0], the 1^st smallest negative integer is -3.
/// For [0, -3], the 1^st smallest negative integer is -3.
///  
/// Constraints:
///
/// 	n == nums.length
/// 	1 <= n <= 10^5
/// 	1 <= k <= n
/// 	1 <= x <= k
/// 	-50 <= nums[i] <= 50
pub struct Solution {}

// problem: https://leetcode.com/problems/sliding-subarray-beauty/
// discuss: https://leetcode.com/problems/sliding-subarray-beauty/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_subarray_beauty(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut ans = vec![0; nums.len() - k as usize + 1];
        const BIAS: i32 = 50;
        let mut cnt = [0; 101];

        // 先往窗口添加 k-1 个数
        for i in 0..(k as usize - 1) {
            cnt[(nums[i] + BIAS) as usize] += 1;
        }

        for i in (k as usize - 1)..nums.len() {
            println!(" 当前元素{:?}", nums[i]);
            cnt[(nums[i] + BIAS) as usize] += 1;
            let mut num = x;
            for v in -50..0 {
                num -= cnt[(v + BIAS) as usize];
                if num <= 0 {
                    println!(" 找到第{:?}个负数{:?}", x, v);
                    ans[i - (k as usize - 1)] = v;
                    break;
                }
            }
            println!(" 减少这个元素的数量{:?}", nums[i - (k as usize - 1)]);
            cnt[(nums[i - (k as usize - 1)] + BIAS) as usize] -= 1;
        }
        println!(" 窗口内的元素{:?}", ans);
        ans
        // for 进入窗口
        // 找到
        // 离开窗口
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2653() {
        let nums = vec![1, -1, -3, -2, 3];
        let k = 3;
        let x = 2;
        let ans = vec![-1, -2, -2];
        assert_eq!(Solution::get_subarray_beauty(nums, k, x), ans);
    }
    #[test]
    fn test_2653_2() {
        let nums = vec![-3, 1, 2, -3, 0, -3];
        let k = 2;
        let x = 1;
        let ans = vec![-3, -3, -3, -3, -3];
        assert_eq!(Solution::get_subarray_beauty(nums, k, x), ans);
    }
}

/// [34] Find First and Last Position of Element in Sorted Array
///
/// Given an array of integers nums sorted in non-decreasing order, find the
/// starting and ending position of a given target value. If target is not found
/// in the array, return [-1, -1]. You must write an algorithm with O(log n)
/// runtime complexity.  
/// <strong class="example">Example 1:
/// Input: nums = [5,7,7,8,8,10], target = 8
/// Output: [3,4]
/// <strong class="example">Example 2:
/// Input: nums = [5,7,7,8,8,10], target = 6
/// Output: [-1,-1]
/// <strong class="example">Example 3:
/// Input: nums = [], target = 0
/// Output: [-1,-1]
///  
/// Constraints:
///
/// 	0 <= nums.length <= 10^5
/// 	-10^9 <= nums[i] <= 10^9
/// 	nums is a non-decreasing array.
/// 	-10^9 <= target <= 10^9
pub struct Solution {}

// problem: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
// discuss: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let start = Self::low_bound(&nums, target);
        if start == nums.len() as i32 || nums[start as usize] != target {
            return vec![-1, -1];
        }
        let end = Self::low_bound_2(&nums, target + 1);
        vec![start, end - 1]
    }
    // 左闭右闭写法
    fn low_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        // 闭区间[left,right]
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1; // [mid+1,right]
            } else {
                right = mid - 1; // [left,mid-1]
            }
        }
        left as _
    }
    // 左闭右开
    fn low_bound_1(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        // 左闭右开区间[left,right)
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1; // [mid+1,right)
            } else {
                right = mid; // [left,mid)
            }
        }
        left as _
    }

    // 左开右开
    fn low_bound_2(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = -1;
        let mut right = nums.len() as i32;
        // 开区间(left,right)
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] < target {
                left = mid; // (mid,right)
            } else {
                right = mid; // (left,mid)
            }
        }
        right as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_34() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let res = vec![3, 4];
        assert_eq!(Solution::search_range(nums, target), res);
    }
}

use std::num;

/// [33] Search in Rotated Sorted Array
///
/// There is an integer array nums sorted in ascending order (with distinct
/// values). Prior to being passed to your function, nums is possibly left
/// rotated at an unknown index k (1 <= k < nums.length) such that the resulting
/// array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ...,
/// nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be left rotated
/// by 3 indices and become [4,5,6,7,0,1,2]. Given the array nums after the
/// possible rotation and an integer target, return the index of target if it is
/// in nums, or -1 if it is not in nums. You must write an algorithm with O(log
/// n) runtime complexity.  
/// <strong class="example">Example 1:
/// Input: nums = [4,5,6,7,0,1,2], target = 0
/// Output: 4
/// <strong class="example">Example 2:
/// Input: nums = [4,5,6,7,0,1,2], target = 3
/// Output: -1
/// <strong class="example">Example 3:
/// Input: nums = [1], target = 0
/// Output: -1
///  
/// Constraints:
///
/// 	1 <= nums.length <= 5000
/// 	-10^4 <= nums[i] <= 10^4
/// 	All values of nums are unique.
/// 	nums is an ascending array that is possibly rotated.
/// 	-10^4 <= target <= 10^4
pub struct Solution {}

// problem: https://leetcode.com/problems/search-in-rotated-sorted-array/
// discuss: https://leetcode.com/problems/search-in-rotated-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut n = nums.len();
        let mut l = 0;
        let mut r = n;
        while l < r {
            let mid = l + (r - l) / 2;
            let x = nums[mid];
            if target > nums[n - 1] && x <= nums[n - 1] {
                r = mid;
            } else if target <= nums[n - 1] && x > nums[n - 1] {
                l = mid + 1;
            } else if x >= target {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        if nums[r] == target { r as _ } else { -1 }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_33() {}
}

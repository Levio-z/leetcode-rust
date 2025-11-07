use core::num;

/// [26] Remove Duplicates from Sorted Array
///
/// Given an integer array nums sorted in non-decreasing order, remove the duplicates <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> such that each unique element appears only once. The relative order of the elements should be kept the same.
/// Consider the number of unique elements in nums to be k​​​​​​​​​​​​​​. <meta
/// charset="UTF-8" />After removing duplicates, return the number of unique
/// elements k. <meta charset="UTF-8" />The first k elements of nums should
/// contain the unique numbers in sorted order. The remaining elements beyond
/// index k - 1 can be ignored. Custom Judge:
/// The judge will test your solution with the following code:
///
/// int[] nums = [...]; // Input array
/// int[] expectedNums = [...]; // The expected answer with correct length
/// int k = removeDuplicates(nums); // Calls your implementation
/// assert k == expectedNums.length;
/// for (int i = 0; i < k; i++) {
///     assert nums[i] == expectedNums[i];
/// }
///
/// If all assertions pass, then your solution will be accepted.
///  
/// <strong class="example">Example 1:
///
/// Input: nums = [1,1,2]
/// Output: 2, nums = [1,2,_]
/// Explanation: Your function should return k = 2, with the first two elements
/// of nums being 1 and 2 respectively. It does not matter what you leave beyond
/// the returned k (hence they are underscores).
///
/// <strong class="example">Example 2:
///
/// Input: nums = [0,0,1,1,1,2,2,3,3,4]
/// Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
/// Explanation: Your function should return k = 5, with the first five elements
/// of nums being 0, 1, 2, 3, and 4 respectively. It does not matter what you
/// leave beyond the returned k (hence they are underscores).
///
///  
/// Constraints:
///
/// 1 <= nums.length <= 3 * 10^4
/// -100 <= nums[i] <= 100
/// nums is sorted in non-decreasing order.
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-duplicates-from-sorted-array/
// discuss: https://leetcode.com/problems/remove-duplicates-from-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut remain = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[remain] = nums[i];
                remain += 1;
            }
        }

        nums.truncate(remain);
        remain as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_26() {
        assert_eq!(Solution::remove_duplicates(&mut vec![]), 0);
        let mut vec1 = vec![1, 1, 1, 1, 3];
        assert_eq!(Solution::remove_duplicates(&mut vec1), 2);
        assert_eq!(vec1, vec![1, 3]);
        let mut vec2 = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut vec2), 2);
        assert_eq!(vec2, vec![1, 2]);
    }
}

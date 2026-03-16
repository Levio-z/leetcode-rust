/// [162] Find Peak Element
///
/// A peak element is an element that is strictly greater than its neighbors.
/// Given a 0-indexed integer array nums, find a peak element, and return its
/// index. If the array contains multiple peaks, return the index to any of the
/// peaks. You may imagine that nums[-1] = nums[n] = -&infin;. In other words,
/// an element is always considered to be strictly greater than a neighbor that
/// is outside the array. You must write an algorithm that runs in O(log n)
/// time.  
/// <strong class="example">Example 1:
///
/// Input: nums = [1,2,3,1]
/// Output: 2
/// Explanation: 3 is a peak element and your function should return the index
/// number 2. <strong class="example">Example 2:
///
/// Input: nums = [1,2,1,3,5,6,4]
/// Output: 5
/// Explanation: Your function can return either index number 1 where the peak
/// element is 2, or index number 5 where the peak element is 6.  
/// Constraints:
///
/// 	1 <= nums.length <= 1000
/// 	-2^31 <= nums[i] <= 2^31 - 1
/// 	nums[i] != nums[i + 1] for all valid i.
pub struct Solution {}

// problem: https://leetcode.com/problems/find-peak-element/
// discuss: https://leetcode.com/problems/find-peak-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l: i32 = -1;
        let mut r = nums.len() - 1;
        while l + 1 < r as i32 {
            let mut mid = (l + (r as i32 - l) / 2) as usize;
            if nums[mid] > nums[mid + 1] {
                r = mid;
            } else {
                l = mid as i32;
            }
        }
        r as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_162_example1() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
    }

    #[test]
    fn test_162_example2() {
        let result = Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]);
        assert!(result == 1 || result == 5);
    }

    #[test]
    fn test_162_single_element() {
        assert_eq!(Solution::find_peak_element(vec![1]), 0);
    }

    #[test]
    fn test_162_two_elements_ascending() {
        assert_eq!(Solution::find_peak_element(vec![1, 2]), 1);
    }

    #[test]
    fn test_162_two_elements_descending() {
        assert_eq!(Solution::find_peak_element(vec![2, 1]), 0);
    }

    #[test]
    fn test_162_descending_array() {
        assert_eq!(Solution::find_peak_element(vec![5, 4, 3, 2, 1]), 0);
    }

    #[test]
    fn test_162_ascending_array() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 4, 5]), 4);
    }
}

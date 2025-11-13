/// [303] Range Sum Query - Immutable
///
/// Given an integer array nums, handle multiple queries of the following type:
/// <ol>
/// <li>Calculate the sum of the elements of nums between indices left and
/// right inclusive where left <= right.</li>
/// </ol>
/// Implement the NumArray class:
///
/// NumArray(int[] nums) Initializes the object with the integer array nums.
/// int sumRange(int left, int right) Returns the sum of the elements of nums
/// between indices left and right inclusive (i.e. nums[left] + nums[left + 1] +
/// ... + nums[right]).
///
///  
/// <strong class="example">Example 1:
///
/// Input
/// ["NumArray", "sumRange", "sumRange", "sumRange"]
/// [[[-2, 0, 3, -5, 2, -1]], [0, 2], [2, 5], [0, 5]]
/// Output
/// [null, 1, -1, -3]
/// Explanation
/// NumArray numArray = new NumArray([-2, 0, 3, -5, 2, -1]);
/// numArray.sumRange(0, 2); // return (-2) + 0 + 3 = 1
/// numArray.sumRange(2, 5); // return 3 + (-5) + 2 + (-1) = -1
/// numArray.sumRange(0, 5); // return (-2) + 0 + 3 + (-5) + 2 + (-1) = -3
///
///  
/// Constraints:
///
///     1 <= nums.length <= 10^4
///     -10^5 <= nums[i] <= 10^5
///     0 <= left <= right < nums.length
///     At most 10^4 calls will be made to sumRange.
pub struct Solution {}

// problem: https://leetcode.com/problems/range-sum-query-immutable/
// discuss: https://leetcode.com/problems/range-sum-query-immutable/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct NumArray {
    p: Vec<i32>,
}

/// `&self` means the method takes an immutable reference.
/// If you need a mutable reference, change it to `&mut self` instead.
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut p = vec![0; nums.len() + 1];
        for (i, &x) in nums.iter().enumerate() {
            // p[0] = 0
            // p[1] = nums[0]+p[0]
            // p[2] = nums[1]+p[1]
            // p[3] = nums[2]+p[2]
            // ...
            // p[nums.len()+1] = nums[nums.len()-1]+p[nums.len()]
            p[i + 1] = p[i] + x;
        }
        Self { p }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.p[right as usize + 1] - self.p[left as usize]
    }
}

/// Your NumArray object will be instantiated and called as such:
/// let obj = NumArray::new(nums);
/// let ret_1: i32 = obj.sum_range(left, right);
/// submission codes end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_303() {
        let nums = vec![-2, 0, 3, -5, 2, -1];
        let num_array = NumArray::new(nums);
        assert_eq!(num_array.sum_range(0, 2), 1);
        assert_eq!(num_array.sum_range(2, 5), -1);
        assert_eq!(num_array.sum_range(0, 5), -3);
    }
    #[test]
    fn test_edge_cases() {
        // 单元素数组
        let nums1 = vec![5];
        let na1 = NumArray::new(nums1);
        assert_eq!(na1.sum_range(0, 0), 5);

        // 全正数数组
        let nums2 = vec![1, 2, 3, 4, 5];
        let na2 = NumArray::new(nums2);
        assert_eq!(na2.sum_range(0, 4), 15);
        assert_eq!(na2.sum_range(1, 3), 9);

        // 全负数数组
        let nums3 = vec![-1, -2, -3, -4, -5];
        let na3 = NumArray::new(nums3);
        assert_eq!(na3.sum_range(0, 4), -15);
    }
}

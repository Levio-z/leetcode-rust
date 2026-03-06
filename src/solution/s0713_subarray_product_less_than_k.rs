/// [713] Subarray Product Less Than K
///
/// Given an array of integers nums and an integer k, return the number of
/// contiguous subarrays where the product of all the elements in the subarray
/// is strictly less than k.  
/// <strong class="example">Example 1:
///
/// Input: nums = [10,5,2,6], k = 100
/// Output: 8
/// Explanation: The 8 subarrays that have product less than 100 are:
/// [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]
/// Note that [10, 5, 2] is not included as the product of 100 is not strictly
/// less than k.
///
/// <strong class="example">Example 2:
///
/// Input: nums = [1,2,3], k = 0
/// Output: 0
///
///  
/// Constraints:
///
///     1 <= nums.length <= 3 * 10^4
///     1 <= nums[i] <= 1000
///     0 <= k <= 10^6
pub struct Solution {}

// problem: https://leetcode.com/problems/subarray-product-less-than-k/
// discuss: https://leetcode.com/problems/subarray-product-less-than-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }
        let mut ans = 0;
        let mut prod = 1;
        let mut l = 0;
        for (r, &x) in nums.iter().enumerate() {
            prod *= x;
            while prod >= k {
                prod /= nums[l];
                l += 1;
            }
            ans += (r - l + 1) as i32;
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_713() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );
    }
}

/// [42] Trapping Rain Water
///
/// Given n non-negative integers representing an elevation map where the width
/// of each bar is 1, compute how much water it can trap after raining.  
/// <strong class="example">Example 1:
/// <img src="https://assets.leetcode.com/uploads/2018/10/22/rainwatertrap.png" style="width: 412px; height: 161px;" />
/// Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
/// Output: 6
/// Explanation: The above elevation map (black section) is represented by array
/// [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue
/// section) are being trapped.
///
/// <strong class="example">Example 2:
///
/// Input: height = [4,2,0,3,2,5]
/// Output: 9
///
///  
/// Constraints:
///
/// 	n == height.length
/// 	1 <= n <= 2 * 10^4
/// 	0 <= height[i] <= 10^5
pub struct Solution {}

// problem: https://leetcode.com/problems/trapping-rain-water/
// discuss: https://leetcode.com/problems/trapping-rain-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut pre_max = vec![0; n];
        pre_max[0] = height[0];
        for i in 1..n {
            pre_max[i] = pre_max[i - 1].max(height[i]);
        }
        let mut suf_max = vec![0; n];
        suf_max[n - 1] = height[n - 1];
        for i in (0..n - 1).rev() {
            suf_max[i] = suf_max[i + 1].max(height[i]);
        }

        let mut ans = 0;
        for i in 0..n {
            ans += pre_max[i].min(suf_max[i]) - height[i];
        }
        ans
    }
    pub fn trap2(height: Vec<i32>) -> i32 {
        // 空间复杂度 O(1)
        // 时间复杂度 O(n)
        let n = height.len();
        let mut ans = 0;
        let mut l = 0;
        let mut r = n - 1;
        let mut pre_max = 0;
        let mut suf_max = 0;
        while l <= r {
            pre_max = pre_max.max(height[l]);
            suf_max = suf_max.max(height[r]);
            if pre_max < suf_max {
                ans += pre_max - height[l];
                l += 1;
            } else {
                ans += suf_max - height[r];
                r -= 1;
            }
        }
        ans
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_42() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}

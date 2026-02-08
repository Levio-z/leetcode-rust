use std::f128::consts::E;

/// [15] 3Sum
///
/// Given an integer array nums, return all the triplets [nums[i], nums[j],
/// nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] +
/// nums[k] == 0. Notice that the solution set must not contain duplicate
/// triplets.  
/// <strong class="example">Example 1:
///
/// Input: nums = [-1,0,1,2,-1,-4]
/// Output: [[-1,-1,2],[-1,0,1]]
/// Explanation:
/// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
/// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
/// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
/// The distinct triplets are [-1,0,1] and [-1,-1,2].
/// Notice that the order of the output and the order of the triplets does not
/// matter.
///
/// <strong class="example">Example 2:
///
/// Input: nums = [0,1,1]
/// Output: []
/// Explanation: The only possible triplet does not sum up to 0.
///
/// <strong class="example">Example 3:
///
/// Input: nums = [0,0,0]
/// Output: [[0,0,0]]
/// Explanation: The only possible triplet sums up to 0.
///
///  
/// Constraints:
///
///     3 <= nums.length <= 3000
///     -10^5 <= nums[i] <= 10^5
pub struct Solution {}

// problem: https://leetcode.com/problems/3sum/
// discuss: https://leetcode.com/problems/3sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        // 三元组的顺序并不重要
        // i<j<k
        // 枚举 i，转换成立两数之和=target-nums[i]
        let n = nums.len();
        let mut res = vec![];
        for i in 0..n - 2 {
            // 跳过重复的元素
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            if i >= n - 2 {
                break;
            }
            if nums[i] + nums[i + 1] + nums[i + 2] > 0 {
                break;
            }
            if nums[i] + nums[n - 2] + nums[n - 1] < 0 {
                continue;
            }
            let mut j = i + 1;
            let mut k = n - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == 0 {
                    res.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    // j<k保证j不会超过k导致越界
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                    k -= 1;
                    while j < k && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                } else if sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        res
    }
    pub fn three_sum2(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        // 三元组的顺序并不重要
        // i<j<k
        // 枚举 i，转换成立两数之和=target-nums[i]
        let n = nums.len();
        let mut res = vec![];
        let mut i = 0;
        while i < n - 2 {
            // 跳过重复的元素
            while i > 0 && nums[i] == nums[i - 1] {
                i += 1;
            }
            if i >= n - 2 {
                break;
            }
            if nums[i] + nums[i + 1] + nums[i + 2] > 0 {
                break;
            }
            if nums[i] + nums[n - 2] + nums[n - 1] < 0 {
                i += 1;
                continue;
            }
            let mut j = i + 1;
            let mut k = n - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                match sum {
                    0 => {
                        res.push(vec![nums[i], nums[j], nums[k]]);
                        j += 1;
                        // j<k保证j不会超过k导致越界
                        while j < k && nums[j] == nums[j - 1] {
                            j += 1;
                        }
                        k -= 1;
                        while j < k && nums[k] == nums[k + 1] {
                            k -= 1;
                        }
                    }
                    sum if sum < 0 => j += 1,
                    _ => k -= 1,
                }
            }
            i += 1;
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = Solution::three_sum(nums);
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn test_15_2() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = Solution::three_sum2(nums);
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }
}

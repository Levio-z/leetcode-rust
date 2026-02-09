/// [18] 4Sum
///
/// Given an array nums of n integers, return an array of all the unique
/// quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:
///
/// 	0 <= a, b, c, d < n
/// 	a, b, c, and d are distinct.
/// 	nums[a] + nums[b] + nums[c] + nums[d] == target
///
/// You may return the answer in any order.
///  
/// <strong class="example">Example 1:
///
/// Input: nums = [1,0,-1,0,-2,2], target = 0
/// Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
///
/// <strong class="example">Example 2:
///
/// Input: nums = [2,2,2,2,2], target = 8
/// Output: [[2,2,2,2]]
///
///  
/// Constraints:
///
/// 	1 <= nums.length <= 200
/// 	-10^9 <= nums[i] <= 10^9
/// 	-10^9 <= target <= 10^9
pub struct Solution {}

// problem: https://leetcode.com/problems/4sum/
// discuss: https://leetcode.com/problems/4sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = vec![];
        let target = target as i64;
        for a in 0..n.saturating_sub(3) {
            let x = nums[a];
            if a > 0 && nums[a - 1] == x {
                continue;
            }
            // 省略两个优化
            // 最小减枝，前面最小的四个数相加都是大于target的，那么不用继续循环了，
            if (x + nums[a + 1]) as i64 + (nums[a + 2] + nums[a + 3]) as i64 > target {
                break;
            }

            if ((x + nums[n - 3]) as i64 + (nums[n - 2] + nums[n - 1]) as i64) < target {
                continue;
            }
            // 后面都是大于target的 x 与后面最大的三个数相加还是小于target
            // continue本次循环
            for b in a + 1..n.saturating_sub(2) {
                let y = nums[b];
                if b > a + 1 && nums[b - 1] == y {
                    continue;
                }
                // 同样省略两个优化
                let mut c = b + 1;
                let mut d = n - 1;
                while c < d {
                    let s = (x + y) as i64 + (nums[c] + nums[d]) as i64;
                    if s < target {
                        c += 1;
                    } else if s > target {
                        d -= 1;
                    } else {
                        ans.push(vec![x, y, nums[c], nums[d]]);
                        c += 1;
                        d -= 1;
                        while c < d && nums[c] == nums[c - 1] {
                            c += 1;
                        }
                        while c < d && nums[d] == nums[d + 1] {
                            d -= 1;
                        }
                    }
                }
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
    fn test_18() {}
}

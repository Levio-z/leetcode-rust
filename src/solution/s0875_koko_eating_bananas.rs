/// [875] Koko Eating Bananas
///
/// Koko loves to eat bananas. There are n piles of bananas, the i^th pile has
/// piles[i] bananas. The guards have gone and will come back in h hours.
/// Koko can decide her bananas-per-hour eating speed of k. Each hour, she
/// chooses some pile of bananas and eats k bananas from that pile. If the pile
/// has less than k bananas, she eats all of them instead and will not eat any
/// more bananas during this hour. Koko likes to eat slowly but still wants to
/// finish eating all the bananas before the guards return. Return the minimum
/// integer k such that she can eat all the bananas within h hours.  
/// <strong class="example">Example 1:
///
/// Input: piles = [3,6,7,11], h = 8
/// Output: 4
///
/// <strong class="example">Example 2:
///
/// Input: piles = [30,11,23,4,20], h = 5
/// Output: 30
///
/// <strong class="example">Example 3:
///
/// Input: piles = [30,11,23,4,20], h = 6
/// Output: 23
///
///  
/// Constraints:
///
/// 	1 <= piles.length <= 10^4
/// 	piles.length <= h <= 10^9
/// 	1 <= piles[i] <= 10^9
pub struct Solution {}

// problem: https://leetcode.com/problems/koko-eating-bananas/
// discuss: https://leetcode.com/problems/koko-eating-bananas/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let check = |k: i32| -> bool {
            let mut sum = 0;
            for &p in &piles {
                sum += (p + k - 1) / k;
                if sum > h {
                    return false;
                }
            }
            sum <= h
        };

        let mut left = 0;
        let mut right = *piles.iter().max().unwrap();
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if check(mid) {
                //  小于等于h说明这个东西太大了，得往左边找
                right = mid;
            } else {
                // 大于h说明这个东西太小了，得往右边找
                left = mid;
            }
        }
        right
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_875_example1() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        assert_eq!(Solution::min_eating_speed(piles, h), 4);
    }

    #[test]
    fn test_875_example2() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        assert_eq!(Solution::min_eating_speed(piles, h), 30);
    }

    #[test]
    fn test_875_example3() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;
        assert_eq!(Solution::min_eating_speed(piles, h), 23);
    }

    #[test]
    fn test_875_single_pile() {
        let piles = vec![100];
        let h = 100;
        assert_eq!(Solution::min_eating_speed(piles, h), 1);
    }
}

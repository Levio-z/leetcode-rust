use std::cmp::min;

/// [2187] Minimum Time to Complete Trips
///
/// You are given an array time where time[i] denotes the time taken by the i^th
/// bus to complete one trip. Each bus can make multiple trips successively;
/// that is, the next trip can start immediately after completing the current
/// trip. Also, each bus operates independently; that is, the trips of one bus
/// do not influence the trips of any other bus. You are also given an integer
/// totalTrips, which denotes the number of trips all buses should make in
/// total. Return the minimum time required for all buses to complete at least
/// totalTrips trips.  
/// <strong class="example">Example 1:
///
/// Input: time = [1,2,3], totalTrips = 5
/// Output: 3
/// Explanation:
/// - At time t = 1, the number of trips completed by each bus are [1,0,0]. The
///   total number of trips completed is 1 + 0 + 0 = 1.
/// - At time t = 2, the number of trips completed by each bus are [2,1,0]. The
///   total number of trips completed is 2 + 1 + 0 = 3.
/// - At time t = 3, the number of trips completed by each bus are [3,1,1]. The
///   total number of trips completed is 3 + 1 + 1 = 5.
/// So the minimum time needed for all buses to complete at least 5 trips is 3.
///
/// <strong class="example">Example 2:
///
/// Input: time = [2], totalTrips = 1
/// Output: 2
/// Explanation:
/// There is only one bus, and it will complete its first trip at t = 2.
/// So the minimum time needed to complete 1 trip is 2.
///
///  
/// Constraints:
///
/// 	1 <= time.length <= 10^5
/// 	1 <= time[i], totalTrips <= 10^7
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-time-to-complete-trips/
// discuss: https://leetcode.com/problems/minimum-time-to-complete-trips/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let total_trips = total_trips as i64;
        let min_t = *time.iter().min().unwrap() as i64;
        let mut left = min_t - 1;
        let mut right = min_t * total_trips;
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            let mut sum = 0;
            for &t in &time {
                sum += mid / t as i64;
            }
            if sum >= total_trips {
                right = mid;
            } else {
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
    fn test_2187() {
        assert_eq!(Solution::minimum_time(vec![1, 2, 3], 5), 3);
        assert_eq!(Solution::minimum_time(vec![2], 1), 2);
        assert_eq!(Solution::minimum_time(vec![1], 1), 1);
        assert_eq!(Solution::minimum_time(vec![1], 10), 10);
        assert_eq!(Solution::minimum_time(vec![2, 3, 4], 5), 6);
        assert_eq!(Solution::minimum_time(vec![5, 10, 10], 9), 25);
    }
}

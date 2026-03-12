use std::collections::HashMap;

/// [2080] Range Frequency Queries
///
/// Design a data structure to find the frequency of a given value in a given
/// subarray. The frequency of a value in a subarray is the number of
/// occurrences of that value in the subarray. Implement the RangeFreqQuery
/// class:
///
/// 	RangeFreqQuery(int[] arr) Constructs an instance of the class with the
/// given 0-indexed integer array arr. 	int query(int left, int right, int
/// value) Returns the frequency of value in the subarray arr[left...right].
///
/// A subarray is a contiguous sequence of elements within an array.
/// arr[left...right] denotes the subarray that contains the elements of nums
/// between indices left and right (inclusive).  
/// <strong class="example">Example 1:
///
/// Input
/// ["RangeFreqQuery", "query", "query"]
/// [[[12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]], [1, 2, 4], [0, 11, 33]]
/// Output
/// [null, 1, 2]
/// Explanation
/// RangeFreqQuery rangeFreqQuery = new RangeFreqQuery([12, 33, 4, 56, 22, 2,
/// 34, 33, 22, 12, 34, 56]); rangeFreqQuery.query(1, 2, 4); // return 1. The
/// value 4 occurs 1 time in the subarray [33, 4] rangeFreqQuery.query(0, 11,
/// 33); // return 2. The value 33 occurs 2 times in the whole array.
///
///  
/// Constraints:
///
/// 	1 <= arr.length <= 10^5
/// 	1 <= arr[i], value <= 10^4
/// 	0 <= left <= right < arr.length
/// 	At most 10^5 calls will be made to query
pub struct Solution {}

// problem: https://leetcode.com/problems/range-frequency-queries/
// discuss: https://leetcode.com/problems/range-frequency-queries/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct RangeFreqQuery {
    pos: HashMap<i32, Vec<usize>>,
}

/// `&self` means the method takes an immutable reference.
/// If you need a mutable reference, change it to `&mut self` instead.
impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let mut pos: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &x) in arr.iter().enumerate() {
            pos.entry(x).or_default().push(i);
        }
        Self { pos }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        if let Some(a) = self.pos.get(&value) {
            let l = a.partition_point(|&x| x < left as usize);
            let r = a.partition_point(|&x| x <= right as usize);
            (r - l) as _
        } else {
            0
        }
    }
}

/// Your RangeFreqQuery object will be instantiated and called as such:
/// let obj = RangeFreqQuery::new(arr);
/// let ret_1: i32 = obj.query(left, right, value);

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2080_examples() {
        let arr = vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56];
        let obj = RangeFreqQuery::new(arr);
        assert_eq!(obj.query(1, 2, 4), 1);
        assert_eq!(obj.query(0, 11, 33), 2);
    }

    #[test]
    fn test_2080_various() {
        let arr = vec![1, 2, 2, 3, 2, 1, 4];
        let obj = RangeFreqQuery::new(arr);
        // single element
        assert_eq!(obj.query(0, 0, 1), 1);
        assert_eq!(obj.query(0, 0, 2), 0);
        // whole range
        assert_eq!(obj.query(0, 6, 2), 3);
        // subrange
        assert_eq!(obj.query(1, 4, 2), 3);

        // value not present
        assert_eq!(obj.query(2, 5, 5), 0);
        // left == right
        assert_eq!(obj.query(3, 3, 3), 1);

        // test against previously-buggy condition
        let arr2 = vec![1, 2, 3];
        let obj2 = RangeFreqQuery::new(arr2);
        assert_eq!(obj2.query(1, 2, 2), 1);
    }

    #[test]
    fn test_2080_edge_cases() {
        // minimal length
        let obj = RangeFreqQuery::new(vec![5]);
        assert_eq!(obj.query(0, 0, 5), 1);
        assert_eq!(obj.query(0, 0, 0), 0);
    }
}

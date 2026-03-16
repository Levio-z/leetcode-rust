/// [74] Search a 2D Matrix
///
/// You are given an m x n integer matrix matrix with the following two
/// properties:
///
/// 	Each row is sorted in non-decreasing order.
/// 	The first integer of each row is greater than the last integer of the
/// previous row.
///
/// Given an integer target, return true if target is in matrix or false
/// otherwise. You must write a solution in O(log(m * n)) time complexity.
///  
/// <strong class="example">Example 1:
/// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/05/mat.jpg" style="width: 322px; height: 242px;" />
/// Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
/// Output: true
///
/// <strong class="example">Example 2:
/// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/05/mat2.jpg" style="width: 322px; height: 242px;" />
/// Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
/// Output: false
///
///  
/// Constraints:
///
/// 	m == matrix.length
/// 	n == matrix[i].length
/// 	1 <= m, n <= 100
/// 	-10^4 <= matrix[i][j], target <= 10^4
pub struct Solution {}

// problem: https://leetcode.com/problems/search-a-2d-matrix/
// discuss: https://leetcode.com/problems/search-a-2d-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut l = 0;
        let mut r = m * n;
        while l < r {
            let mid = l + (r - l) / 2;
            let x = matrix[mid / n][mid % n];
            if x == target {
                return true;
            } else if x > target {
                r = mid;
            } else {
                l += 1;
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_74_example1() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(Solution::search_matrix(matrix, 3));
    }

    #[test]
    fn test_74_example2() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(!Solution::search_matrix(matrix, 13));
    }

    #[test]
    fn test_74_single_element_found() {
        assert!(Solution::search_matrix(vec![vec![1]], 1));
    }

    #[test]
    fn test_74_single_element_not_found() {
        assert!(!Solution::search_matrix(vec![vec![1]], 0));
    }

    #[test]
    fn test_74_first_element() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20]];
        assert!(Solution::search_matrix(matrix, 1));
    }

    #[test]
    fn test_74_last_element() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20]];
        assert!(Solution::search_matrix(matrix, 20));
    }

    #[test]
    fn test_74_single_row_found() {
        assert!(Solution::search_matrix(vec![vec![1, 3, 5, 7]], 5));
    }

    #[test]
    fn test_74_single_row_not_found() {
        assert!(!Solution::search_matrix(vec![vec![1, 3, 5, 7]], 4));
    }
}

/// [59] Spiral Matrix II
///
/// Given a positive integer n, generate an n x n matrix filled with elements
/// from 1 to n^2 in spiral order.  
/// <strong class="example">Example 1:
/// <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiraln.jpg" style="width: 242px; height: 242px;" />
/// Input: n = 3
/// Output: [[1,2,3],[8,9,4],[7,6,5]]
///
/// <strong class="example">Example 2:
///
/// Input: n = 1
/// Output: [[1]]
///
///  
/// Constraints:
///
/// 	1 <= n <= 20
pub struct Solution {}

// problem: https://leetcode.com/problems/spiral-matrix-ii/
// discuss: https://leetcode.com/problems/spiral-matrix-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // 右下左上

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; n as usize]; n as usize];
        // 当前位置和方向索引
        let (mut x, mut y, mut di) = (0, 0, 0);

        for val in 1..=n * n {
            ans[x as usize][y as usize] = val;

            // 计算下一步位置
            let next_x = x + DIRS[di].0;
            let next_y = y + DIRS[di].1;

            // 检查是否需要转向：越界或已访问
            if next_x < 0
                || next_y < 0
                || next_x >= n
                || next_y >= n
                || ans[next_x as usize][next_y as usize] != 0
            {
                di = (di + 1) % 4; // 右转90度
            }

            // 移动一步
            x += DIRS[di].0;
            y += DIRS[di].1;
        }
        ans
    }

    pub fn generate_matrix1(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut ans = vec![vec![0; n as _]; n as _];
        let (mut left, mut right, mut top, mut bottom) =
            (0i32, (n - 1) as i32, 0i32, (n - 1) as i32);
        let mut num = 1;
        while left <= right && top <= bottom {
            for col in left..=right {
                ans[top as usize][col as usize] = num;
                num += 1;
            }

            top += 1;
            for row in top..=bottom {
                ans[row as usize][right as usize] = num;
                num += 1;
            }
            right -= 1;

            for col in (left..=right).rev() {
                ans[bottom as usize][col as usize] = num;
                num += 1;
            }
            bottom -= 1;

            for row in (top..=bottom).rev() {
                ans[row as usize][left as usize] = num;
                num += 1;
            }
            left += 1;
        }
        ans
    }
}
// submission codes end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_59() {
        let n = 3;
        let mut matrix = Solution::generate_matrix(n);
        assert_eq!(matrix, vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]);
    }

    #[test]
    fn test_basic_cases() {
        // n=1 边界情况
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);

        // n=2 最小非平凡情况
        assert_eq!(Solution::generate_matrix(2), vec![vec![1, 2], vec![4, 3]]);

        // n=3 标准示例
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }
}

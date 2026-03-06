/// [209] Minimum Size Subarray Sum
///
/// Given an array of positive integers nums and a positive integer target,
/// return the minimal length of a <span
/// data-keyword="subarray-nonempty">subarray</span> whose sum is greater than
/// or equal to target. If there is no such subarray, return 0 instead.  
/// <strong class="example">Example 1:
///
/// Input: target = 7, nums = [2,3,1,2,4,3]
/// Output: 2
/// Explanation: The subarray [4,3] has the minimal length under the problem
/// constraint.
///
/// <strong class="example">Example 2:
///
/// Input: target = 4, nums = [1,4,4]
/// Output: 1
///
/// <strong class="example">Example 3:
///
/// Input: target = 11, nums = [1,1,1,1,1,1,1,1]
/// Output: 0
///
///  
/// Constraints:
///
///     1 <= target <= 10^9
///     1 <= nums.length <= 10^5
///     1 <= nums[i] <= 10^4
///
///  
/// Follow up: If you have figured out the O(n) solution, try coding another
/// solution of which the time complexity is O(n log(n)).
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-size-subarray-sum/
// discuss: https://leetcode.com/problems/minimum-size-subarray-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_sub_array_len1(target: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        // 将答案初始化比较大的一个数，主要方便后面取min
        let mut ans = nums.len() + 1;
        // 数组的元素个数设置为0
        let mut num: usize = 0;
        // 数组的左端点设置为0
        let mut left = 0;
        for (i, &x) in nums.iter().enumerate() {
            sum += x;
            num += 1;
            while (sum >= target) {
                ans = num.min(ans);
                sum -= nums[left];
                left += 1;
                num -= 1;
            }
        }
        if ans > nums.len() { 0 } else { ans as _ }
    }

    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans: usize = n + 1;
        let mut sum = 0; // 子数组元素和
        let mut left = 0; //子数组右端点
        for (right, &x) in nums.iter().enumerate() {
            sum += x;
            while sum >= target {
                ans = ans.min(right - left + 1);
                sum -= nums[left];
                left += 1; //左端点右移
            }
        }
        if ans <= n { ans as i32 } else { 0 }
    }
    pub fn min_sub_array_len2(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans: usize = n + 1;
        let mut sum = 0; // 子数组元素和
        let mut left = 0; //子数组右端点
        for (right, &x) in nums.iter().enumerate() {
            // 向右移动添加新元素
            sum += x;
            // 开始移动左端点
            // 为什么不用判断left<=right,因为left==right之后，这里等于0了，
            // 所以这个while条件不成立，所以left不会继续往右边移动了
            while sum - nums[left] >= target {
                sum -= nums[left];
                // left 至多加到n就结束了，每次都是从上个循环结束的位置开始的，等于是连续的
                left += 1;
            }
            if sum >= target {
                // 怎么判断是不是需要加1，如果right=left，是不是指向一个数
                ans = ans.min(right - left + 1);
            }
        }
        if ans <= n { ans as i32 } else { 0 }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    // 定义测试用例结构体
    struct TestCase {
        target: i32,
        nums: Vec<i32>,
        expected: i32,
    }

    // 定义测试函数类型
    type SolutionFn = fn(i32, Vec<i32>) -> i32;

    // 通用测试函数
    fn run_test_cases(method: SolutionFn, test_cases: &[TestCase], method_name: &str) {
        for (i, tc) in test_cases.iter().enumerate() {
            let result = method(tc.target, tc.nums.clone());
            assert_eq!(
                result,
                tc.expected,
                "{}: Test case {} failed: target={}, nums={:?}, expected={}, got={}",
                method_name,
                i + 1,
                tc.target,
                tc.nums,
                tc.expected,
                result
            );
        }
    }

    // 基础测试用例
    fn basic_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                target: 7,
                nums: vec![2, 3, 1, 2, 4, 3],
                expected: 2,
            },
            TestCase {
                target: 4,
                nums: vec![1, 4, 4],
                expected: 1,
            },
            TestCase {
                target: 11,
                nums: vec![1, 1, 1, 1, 1, 1, 1, 1],
                expected: 0,
            },
        ]
    }

    // 边界测试用例
    fn edge_test_cases() -> Vec<TestCase> {
        vec![
            // 单个元素满足条件
            TestCase {
                target: 5,
                nums: vec![5],
                expected: 1,
            },
            // 单个元素不满足条件
            TestCase {
                target: 6,
                nums: vec![5],
                expected: 0,
            },
            // 需要整个数组才能满足条件
            TestCase {
                target: 15,
                nums: vec![1, 2, 3, 4, 5],
                expected: 5,
            },
            // 数组开头就满足条件
            TestCase {
                target: 3,
                nums: vec![3, 1, 1, 1],
                expected: 1,
            },
            // 数组末尾满足条件
            TestCase {
                target: 3,
                nums: vec![1, 1, 1, 3],
                expected: 1,
            },
            // 重复元素
            TestCase {
                target: 8,
                nums: vec![2, 2, 2, 2, 2],
                expected: 4,
            },
        ]
    }

    // 大规模测试用例
    fn large_test_cases() -> Vec<TestCase> {
        vec![TestCase {
            target: 100000,
            nums: vec![1; 100000],
            expected: 100000,
        }]
    }

    // ==================== min_sub_array_len1 测试 ====================
    #[test]
    fn test_min_sub_array_len1_basic() {
        run_test_cases(
            Solution::min_sub_array_len1,
            &basic_test_cases(),
            "min_sub_array_len1",
        );
    }

    #[test]
    fn test_min_sub_array_len1_edge() {
        run_test_cases(
            Solution::min_sub_array_len1,
            &edge_test_cases(),
            "min_sub_array_len1",
        );
    }

    #[test]
    fn test_min_sub_array_len1_large() {
        run_test_cases(
            Solution::min_sub_array_len1,
            &large_test_cases(),
            "min_sub_array_len1",
        );
    }

    // ==================== min_sub_array_len 测试 ====================
    #[test]
    fn test_min_sub_array_len_basic() {
        run_test_cases(
            Solution::min_sub_array_len,
            &basic_test_cases(),
            "min_sub_array_len",
        );
    }

    #[test]
    fn test_min_sub_array_len_edge() {
        run_test_cases(
            Solution::min_sub_array_len,
            &edge_test_cases(),
            "min_sub_array_len",
        );
    }

    #[test]
    fn test_min_sub_array_len_large() {
        run_test_cases(
            Solution::min_sub_array_len,
            &large_test_cases(),
            "min_sub_array_len",
        );
    }

    // ==================== min_sub_array_len2 测试 ====================
    #[test]
    fn test_min_sub_array_len2_basic() {
        run_test_cases(
            Solution::min_sub_array_len2,
            &basic_test_cases(),
            "min_sub_array_len2",
        );
    }

    #[test]
    fn test_min_sub_array_len2_edge() {
        run_test_cases(
            Solution::min_sub_array_len2,
            &edge_test_cases(),
            "min_sub_array_len2",
        );
    }

    #[test]
    fn test_min_sub_array_len2_large() {
        run_test_cases(
            Solution::min_sub_array_len2,
            &large_test_cases(),
            "min_sub_array_len2",
        );
    }

    // ==================== 三个方法结果一致性测试 ====================
    #[test]
    fn test_all_methods_consistent() {
        let all_cases: Vec<TestCase> = basic_test_cases()
            .into_iter()
            .chain(edge_test_cases().into_iter())
            .chain(large_test_cases().into_iter())
            .collect();

        for tc in all_cases {
            let result1 = Solution::min_sub_array_len1(tc.target, tc.nums.clone());
            let result2 = Solution::min_sub_array_len(tc.target, tc.nums.clone());
            let result3 = Solution::min_sub_array_len2(tc.target, tc.nums.clone());

            assert_eq!(
                result1, result2,
                "min_sub_array_len1 and min_sub_array_len differ: target={}, nums={:?}, expected={}",
                tc.target, tc.nums, tc.expected
            );
            assert_eq!(
                result2, result3,
                "min_sub_array_len and min_sub_array_len2 differ: target={}, nums={:?}, expected={}",
                tc.target, tc.nums, tc.expected
            );
        }
    }
}

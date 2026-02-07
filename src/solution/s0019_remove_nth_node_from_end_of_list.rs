/// [19] Remove Nth Node From End of List
///
/// Given the head of a linked list, remove the n^th node from the end of the
/// list and return its head.  
/// <strong class="example">Example 1:
/// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/remove_ex1.jpg" style="width: 542px; height: 222px;" />
/// Input: head = [1,2,3,4,5], n = 2
/// Output: [1,2,3,5]
///
/// <strong class="example">Example 2:
///
/// Input: head = [1], n = 1
/// Output: []
///
/// <strong class="example">Example 3:
///
/// Input: head = [1,2], n = 1
/// Output: [1]
///
///  
/// Constraints:
///
///     The number of nodes in the list is sz.
///     1 <= sz <= 30
///     0 <= Node.val <= 100
///     1 <= n <= sz
///
///  
/// Follow up: Could you do this in one pass?
pub struct Solution {}

use rand::rand_core::le;

use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/remove-nth-node-from-end-of-list/
// discuss: https://leetcode.com/problems/remove-nth-node-from-end-of-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let dummy = ListNode { val: 0, next: head };
        let mut left = &dummy;
        let mut right = &dummy;

        for _ in 0..n {
            right = right.next.as_ref()?;
        }
        while let Some(node) = &right.next {
            left = left.next.as_ref()?;
            right = node;
        }
        #[allow(mutable_transmutes)]
        let left: &mut ListNode = unsafe { std::mem::transmute(left) };
        left.next = left.next.take()?.next;
        dummy.next
    }
    pub fn remove_nth_from_end_raw_ptr(
        mut head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        // 左指针直接可变引用
        // 右指针使用裸指针
        let mut right: *const Box<ListNode> = unsafe { &dummy };
        let mut left = &mut dummy;

        // 右指针先走 n 步
        for _ in 0..n {
            unsafe { right = (*right).next.as_ref()? }
        }
        while let Some(next_node) = unsafe { &(*right).next } {
            left = left.next.as_mut()?;
            right = next_node;
        }
        left.next = left.next.as_mut()?.next.take();

        dummy.next
    }
    pub fn remove_nth_from_end_travel(
        mut head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        Self::travel(&mut dummy, n);
        dummy.next
    }
    pub fn travel(noop: &mut Box<ListNode>, n: i32) -> i32 {
        match noop.next {
            Some(ref mut next) => {
                // 选择节点
                // 递归
                let index = 1 + Self::travel(next, n);
                // 回溯，根据递归结果处理当前节点
                if index == n {
                    noop.next = next.next.take();
                }
                index
            }
            // 终止条件
            None => 0,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nth_from_end() {
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
        assert_eq!(Solution::remove_nth_from_end(to_list(vec![1]), 1), None);
    }
    #[test]
    fn test_remove_nth_from_end_raw_ptr() {
        assert_eq!(
            Solution::remove_nth_from_end_raw_ptr(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
        assert_eq!(
            Solution::remove_nth_from_end_raw_ptr(to_list(vec![1]), 1),
            None
        );
    }
    #[test]
    fn test_remove_nth_from_end_travel() {
        assert_eq!(
            Solution::remove_nth_from_end_travel(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
        assert_eq!(
            Solution::remove_nth_from_end_travel(to_list(vec![1]), 1),
            None
        );
    }
}

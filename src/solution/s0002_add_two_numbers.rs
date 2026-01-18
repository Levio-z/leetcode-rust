/// [2] Add Two Numbers
///
/// You are given two non-empty linked lists representing two non-negative
/// integers. The digits are stored in reverse order, and each of their nodes
/// contains a single digit. Add the two numbers and return the sum as a linked
/// list. You may assume the two numbers do not contain any leading zero, except
/// the number 0 itself.  
/// <strong class="example">Example 1:
/// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg" style="width: 483px; height: 342px;" />
/// Input: l1 = [2,4,3], l2 = [5,6,4]
/// Output: [7,0,8]
/// Explanation: 342 + 465 = 807.
///
/// <strong class="example">Example 2:
///
/// Input: l1 = [0], l2 = [0]
/// Output: [0]
///
/// <strong class="example">Example 3:
///
/// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
/// Output: [8,9,9,9,0,0,0,1]
///
///  
/// Constraints:
///
/// 	The number of nodes in each linked list is in the range [1, 100].
/// 	0 <= Node.val <= 9
/// 	It is guaranteed that the list represents a number that does not have
/// leading zeros.
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/add-two-numbers/
// discuss: https://leetcode.com/problems/add-two-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

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
trait AddTwoNumbers {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>>;
}
impl AddTwoNumbers for Solution {
    fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = &mut dummy;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }
            carry = sum / 10;
            cur.next = Some(Box::new(ListNode::new(sum % 10)));
            cur = cur.next.as_mut().unwrap();
        }
        dummy.next.take()
    }
}
// 递归实现
pub struct SolutionRecursive {}
impl SolutionRecursive {
    fn add_two_numbers_recursive(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        val: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2, val) {
            (None, None, 0) => None,
            (None, None, val) => Some(Box::new(ListNode::new(val))),
            (Some(node), None, val) => Some(Box::new(ListNode {
                val: (node.val + val) % 10,
                next: Self::add_two_numbers_recursive(node.next, None, (node.val + val) / 10),
            })),
            (None, Some(node), val) => Self::add_two_numbers_recursive(Some(node), None, val),
            (Some(node1), Some(node2), val) => Some(Box::new(ListNode {
                val: (node1.val + node2.val + val) % 10,
                next: Self::add_two_numbers_recursive(
                    node1.next,
                    node2.next,
                    (node1.val + node2.val + val) / 10,
                ),
            })),
        }
    }
}
impl AddTwoNumbers for SolutionRecursive {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two_numbers_recursive(l1, l2, 0)
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let l1 = to_list(vec![2, 4, 9]);
        let l2 = to_list(vec![5, 6, 4]);
        let res = to_list(vec![7, 0, 4, 1]);
        assert_eq!(Solution::add_two_numbers(l1, l2), res);
    }
    #[test]
    fn test_2_recursive() {
        let l1 = to_list(vec![2, 4, 9]);
        let l2 = to_list(vec![5, 6, 4]);
        let res = to_list(vec![7, 0, 4, 1]);
        assert_eq!(SolutionRecursive::add_two_numbers(l1, l2), res);
    }
}

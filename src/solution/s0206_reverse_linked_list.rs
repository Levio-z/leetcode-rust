/// [206] Reverse Linked List
///
/// Given the head of a singly linked list, reverse the list, and return the
/// reversed list.  
/// <strong class="example">Example 1:
/// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev1ex1.jpg" style="width: 542px; height: 222px;" />
/// Input: head = [1,2,3,4,5]
/// Output: [5,4,3,2,1]
///
/// <strong class="example">Example 2:
/// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev1ex2.jpg" style="width: 182px; height: 222px;" />
/// Input: head = [1,2]
/// Output: [2,1]
///
/// <strong class="example">Example 3:
///
/// Input: head = []
/// Output: []
///
///  
/// Constraints:
///
/// The number of nodes in the list is the range [0, 5000].
/// -5000 <= Node.val <= 5000
///
///  
/// Follow up: A linked list can be reversed either iteratively or recursively.
/// Could you implement both?
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/reverse-linked-list/
// discuss: https://leetcode.com/problems/reverse-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn reverse_list(mut cur: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 初始分为 pre 已处理好的 cur正在处理中的节点
        let mut pre = None;
        while let Some(mut node) = cur {
            // 分离处理中的节点和未处理好的节点
            let temp = node.next.take();
            // 处理
            node.next = pre;
            // 维护下个循环信息：
            // 处理好的节点，作为下个循环的pre
            // 未处理好的节点，作为下个循环的cur
            pre = Some(node);
            cur = temp;
        }
        pre
    }
    pub fn reverse_list2(cur: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn reverse(
            cur: Option<Box<ListNode>>,
            pre: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match cur {
                None => pre,
                Some(mut node) => {
                    let temp = node.next.take();
                    node.next = pre;
                    reverse(temp, Some(node))
                }
            }
        }
        reverse(cur, None)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_206() {
        assert_eq!(
            Solution::reverse_list(linked![1, 2, 3, 4, 5]),
            linked![5, 4, 3, 2, 1]
        );
    }
    #[test]
    fn test_206_2() {
        assert_eq!(
            Solution::reverse_list2(linked![1, 2, 3, 4, 5]),
            linked![5, 4, 3, 2, 1]
        );
    }
}

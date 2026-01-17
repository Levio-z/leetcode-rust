/// [203] Remove Linked List Elements
///
/// Given the head of a linked list and an integer val, remove all the nodes
/// the linked list that has Node.val == val, and return the new head.
/// <strong class="example">Example 1:
/// <img alt="" src="https://assets.leetcode.com/uploads/2021/03/06/removelinked-list.jpg" style="width: 500px; height: 142px;" />
/// Input: head = [1,2,6,3,4,5,6], val = 6
/// Output: [1,2,3,4,5]
///
/// <strong class="example">Example 2:
///
/// Input: head = [], val = 1
/// Output: []
///
/// <strong class="example">Example 3:
///
/// Input: head = [7,7,7,7], val = 7
/// Output: []
///
///
/// Constraints:
///
/// The number of nodes in the list is in the range [0, 10^4].
/// 1 <= Node.val <= 50
/// 0 <= val <= 50
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/remove-linked-list-elements/
// discuss: https://leetcode.com/problems/remove-linked-list-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.

impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut p = &mut head;
        while let Some(node) = p {
            if node.val == val {
                *p = node.next.take();
            } else {
                p = &mut p.as_mut().unwrap().next;
            }
        }
        head.take()
    }

    pub fn remove_elements2(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        // dummy产生了一个引用cur
        let mut cur = &mut dummy;

        // 使用dummy引用访问字段，获取cur.next对象的引用,cur.next产生了一个引用，nxt
        while let Some(ref mut nxt) = cur.next {
            // 使用nxt
            if nxt.val == val {
                // 右边 使用nxt
                cur.next = nxt.next.take();
            } else {
                // 获取字段的引用
                cur = cur.next.as_mut().unwrap(); // 继续向后遍历链表
            }
        }
        // dummy的引用生命周期结束
        dummy.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_203() {
        assert_eq!(
            Solution::remove_elements(linked![1, 2, 6, 3, 4, 5, 6], 6),
            linked![1, 2, 3, 4, 5]
        );
        assert_eq!(Solution::remove_elements(linked![], 1), linked![]);
        assert_eq!(Solution::remove_elements(linked![7, 7, 7, 7], 7), linked![]);
    }
}

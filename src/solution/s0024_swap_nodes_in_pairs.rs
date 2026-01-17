/// [24] Swap Nodes in Pairs
///
/// Given a linked list, swap every two adjacent nodes and return its head. You
/// must solve the problem without modifying the values in the list's nodes
/// (i.e., only nodes themselves may be changed.)  
/// <strong class="example">Example 1:
/// <div class="example-block">
/// Input: <span class="example-io">head = [1,2,3,4]</span>
/// Output: <span class="example-io">[2,1,4,3]</span>
/// Explanation:
/// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/swap_ex1.jpg" style="width: 422px; height: 222px;" />
/// </div>
/// <strong class="example">Example 2:
/// <div class="example-block">
/// Input: <span class="example-io">head = []</span>
/// Output: <span class="example-io">[]</span>
/// </div>
/// <strong class="example">Example 3:
/// <div class="example-block">
/// Input: <span class="example-io">head = [1]</span>
/// Output: <span class="example-io">[1]</span>
/// </div>
/// <strong class="example">Example 4:
/// <div class="example-block">
/// Input: <span class="example-io">head = [1,2,3]</span>
/// Output: <span class="example-io">[2,1,3]</span>
/// </div>
///  
/// Constraints:
///
/// The number of nodes in the list is in the range [0, 100].
/// 0 <= Node.val <= 100
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/swap-nodes-in-pairs/
// discuss: https://leetcode.com/problems/swap-nodes-in-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

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
    /// 非递归写法题解：https://leetcode.cn/problems/swap-nodes-in-pairs/solutions/3808522/fei-di-gui-xie-fa-jian-ji-yi-dong-by-zkd-7a34
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 1️⃣ 构造哨兵节点，简化链表操作
        let mut dummy = Box::new(ListNode::new(0));
        // tail 始终指向已处理链表的最后节点
        let mut tail = &mut dummy;

        // 2️⃣ 遍历链表，每次处理一对节点
        while let Some(mut first) = head {
            // 次数必须用显示move，否则会报错：use of partially moved value: `first`
            match first.next.take() {
                // 暂存下一轮待处理的链表
                Some(mut second) => {
                    head = second.next.take();
                    // use of partially moved value: `first`
                    // 交换节点
                    second.next = Some(first);

                    // 拼接到已处理链表
                    tail.next = Some(second);

                    // 更新尾节点，指向当前已处理链表的最后一个节点
                    tail = tail.next.as_mut().unwrap().next.as_mut().unwrap();
                }
                None => {
                    // 如果剩余只有一个节点，直接拼接
                    tail.next = Some(first);
                    break;
                }
            }
        }

        // 返回新的链表头
        dummy.next
    }

    // 递归写法题解：https://leetcode.cn/problems/swap-nodes-in-pairs/solutions/445236/rust-by-raphael-wang-3/
    #[allow(clippy::bind_instead_of_map)]
    pub fn swap_pairs_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut n| {
            // n 被部分move，不能直接使用n
            match n.next {
                None => Some(n),
                // n.next 被move，不能使用n和n.next
                Some(mut m) => {
                    // n.next被赋值，n又不存在move状态
                    // 递归交换剩余链表
                    n.next = Self::swap_pairs(m.next);
                    // 当前节点对交换
                    m.next = Some(n);
                    Some(m)
                }
            }
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_24_swap_pairs_recursive() {
        assert_eq!(
            Solution::swap_pairs_recursive(to_list(vec![1, 2, 3, 4])),
            to_list(vec![2, 1, 4, 3])
        );
        assert_eq!(
            Solution::swap_pairs_recursive(to_list(vec![])),
            to_list(vec![])
        );
        assert_eq!(
            Solution::swap_pairs_recursive(to_list(vec![1, 2, 3])),
            to_list(vec![2, 1, 3])
        );
        assert_eq!(
            Solution::swap_pairs_recursive(to_list(vec![1])),
            to_list(vec![1])
        );
    }
    #[test]
    fn test_24() {
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4])),
            to_list(vec![2, 1, 4, 3])
        );
        assert_eq!(Solution::swap_pairs(to_list(vec![])), to_list(vec![]));
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3])),
            to_list(vec![2, 1, 3])
        );
        assert_eq!(Solution::swap_pairs(to_list(vec![1])), to_list(vec![1]));
    }
}

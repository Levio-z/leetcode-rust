use std::ptr::addr_of;

use openssl_sys::X509_V_ERR_IP_ADDRESS_MISMATCH;

/// [707] Design Linked List
///
/// Design your implementation of the linked list. You can choose to use a
/// singly or doubly linked list.<br /> A node in a singly linked list should
/// have two attributes: val and next. val is the value of the current node, and
/// next is a pointer/reference to the next node.<br /> If you want to use the
/// doubly linked list, you will need one more attribute prev to indicate the
/// previous node in the linked list. Assume all nodes in the linked list are
/// 0-indexed. Implement the MyLinkedList class:
///
/// MyLinkedList() Initializes the MyLinkedList object.
/// int get(int index) Get the value of the index^th node in the linked list. If
/// the index is invalid, return -1. void addAtHead(int val) Add a node of value
/// val before the first element of the linked list. After the insertion, the
/// new node will be the first node of the linked list. void addAtTail(int val)
/// Append a node of value val as the last element of the linked list.
/// void addAtIndex(int index, int val) Add a node of value val before the
/// index^th node in the linked list. If index equals the length of the linked
/// list, the node will be appended to the end of the linked list. If index is
/// greater than the length, the node will not be inserted.
/// void deleteAtIndex(int index) Delete the index^th node in the linked list,
/// if the index is valid.
///
///  
/// <strong class="example">Example 1:
///
/// Input
/// ["MyLinkedList", "addAtHead", "addAtTail", "addAtIndex", "get",
/// "deleteAtIndex", "get"] [[], [1], [3], [1, 2], [1], [1], [1]]
/// Output
/// [null, null, null, null, 2, null, 3]
/// Explanation
/// MyLinkedList myLinkedList = new MyLinkedList();
/// myLinkedList.addAtHead(1);
/// myLinkedList.addAtTail(3);
/// myLinkedList.addAtIndex(1, 2);    // linked list becomes 1->2->3
/// myLinkedList.get(1);              // return 2
/// myLinkedList.deleteAtIndex(1);    // now the linked list is 1->3
/// myLinkedList.get(1);              // return 3
///
///  
/// Constraints:
///
/// 0 <= index, val <= 1000
/// Please do not use the built-in LinkedList library.
/// At most 2000 calls will be made to get, addAtHead, addAtTail, addAtIndex and
/// deleteAtIndex.
pub struct Solution {}

// problem: https://leetcode.com/problems/design-linked-list/
// discuss: https://leetcode.com/problems/design-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/// 单链表节点结构
///
/// 设计要点：
/// - val: 存储节点的整数值
/// - next: 使用 Option<Box<Node>> 实现可选的下一个节点
/// - Box 指针：确保堆分配和明确的所有权
/// - #[derive(Debug)]：支持调试输出
#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

/// 单链表结构
///
/// 设计要点：
/// - head: 指向链表第一个节点的可选指针
/// - len: 维护链表长度，优化索引操作
/// - 长度缓存：避免每次操作都需要遍历计算长度
#[derive(Debug)]
struct MyLinkedList {
    head: Option<Box<Node>>,
    len: usize,
}

/// `&self` means the method takes an immutable reference.
/// If you need a mutable reference, change it to `&mut self` instead.
impl MyLinkedList {
    /// 构造函数：初始化空链表
    ///
    /// 初始化：空链表，头指针为 None，长度为 0
    fn new() -> Self {
        MyLinkedList { head: None, len: 0 }
    }

    /// 获取节点值（get）
    ///
    /// 算法步骤：
    /// 1. 边界检查：索引是否有效
    /// 2. 指针遍历：从头节点开始逐个移动
    /// 3. 返回值：找到目标节点的值
    ///
    /// 时间复杂度：O(n)，最坏情况需要遍历整个链表
    fn get(&self, index: i32) -> i32 {
        // 快速失败检查：索引有效性验证
        if index < 0 || index >= self.len as i32 {
            return -1;
        }

        let mut p = &self.head;
        let mut i = 0;

        // 遍历到目标节点
        while let Some(node) = p {
            if i == index {
                break;
            }
            i += 1;
            p = &node.next;
        }

        p.as_ref().unwrap().val
    }

    /// 头插法：在链表头部插入节点
    ///
    /// 实现：通过调用 add_at_index(0, val) 实现
    fn add_at_head(&mut self, val: i32) {
        self.add_at_index(0, val);
    }

    /// 尾插法：在链表尾部插入节点
    ///
    /// 实现：通过调用 add_at_index(self.len as i32, val) 实现
    fn add_at_tail(&mut self, val: i32) {
        self.add_at_index(self.len as i32, val);
    }

    /// 在指定位置插入节点
    ///
    /// 插入逻辑：
    /// 1. 找到前驱节点
    /// 2. 创建新节点：val 为指定值，next 为前驱节点的原下一个节点
    /// 3. 更新前驱节点的 next 指向新节点
    /// 4. 增加链表长度
    fn add_at_index(&mut self, index: i32, val: i32) {
        // 快速失败检查：索引有效性验证
        if index < 0 || index > self.len as i32 {
            return;
        }

        // 使用通用辅助方法执行插入操作
        self.find_pre_and_excute(index, |node| {
            node.next = Some(Box::new(Node {
                val,
                next: node.next.take(),
            }));
        });

        self.len += 1;
    }

    /// 删除指定位置的节点
    ///
    /// 删除逻辑：
    /// 1. 找到前驱节点
    /// 2. 将前驱节点的 next 指向被删除节点的下一个节点
    /// 3. 减少链表长度
    fn delete_at_index(&mut self, index: i32) {
        // 快速失败检查：索引有效性验证
        if index < 0 || index >= self.len as i32 {
            return;
        }

        // 使用通用辅助方法执行删除操作
        self.find_pre_and_excute(index, |node| {
            node.next = node.next.as_mut().unwrap().next.take();
        });

        self.len -= 1;
    }

    /// 通用辅助方法：查找前驱节点并执行操作
    ///
    /// 设计思想：
    /// - 虚拟头节点：统一处理头节点的特殊情况
    /// - 函数式参数：使用闭包实现通用的节点操作
    /// - 所有权管理：使用 take() 方法安全转移指针所有权
    ///
    /// 参数：
    /// - index: 目标索引位置
    /// - update: 闭包函数，对找到的前驱节点执行具体操作
    fn find_pre_and_excute(&mut self, index: i32, mut update: impl FnMut(&mut Box<Node>)) {
        // 创建虚拟头节点简化操作
        let mut dummy = Some(Box::new(Node {
            val: 0,
            next: self.head.take(),
        }));

        let mut cur = &mut dummy;
        let mut i = -1;
        let mut pre = None;

        // 查找前驱节点
        while let Some(node) = cur {
            if i + 1 == index {
                pre = Some(cur.as_mut().unwrap());
                break;
            }
            i += 1;
            cur = &mut cur.as_mut().unwrap().next;
        }

        // 执行更新操作
        if let Some(pre) = pre {
            update(pre);
        }

        // 恢复头指针
        self.head = dummy.as_mut().unwrap().next.take();
    }
}

#[cfg(test)]
mod tests {
    use surf::get;

    use super::*;

    /// 基础功能测试
    #[test]
    fn test_707() {
        let mut my_linked_list = MyLinkedList::new();

        // 测试头插法
        my_linked_list.add_at_head(1);
        println!("{:?}", my_linked_list);

        // 测试尾插法
        my_linked_list.add_at_tail(3);
        println!("{:?}", my_linked_list);

        // 测试中间插入
        my_linked_list.add_at_index(1, 2);
        println!("{:?}", my_linked_list);

        // 测试获取节点值
        assert_eq!(my_linked_list.get(1), 2);
        println!("{:?}", my_linked_list);

        // 测试删除节点
        my_linked_list.delete_at_index(1);
        assert_eq!(my_linked_list.get(1), 3);
    }

    /// 边界情况测试：空链表插入
    #[test]
    fn test_708() {
        let mut my_linked_list = MyLinkedList::new();

        // 测试在空链表的索引1处插入（应该插入到位置0）
        // 如果 index 比长度更大，该节点将 不会插入 到链表中。
        my_linked_list.add_at_index(1, 0);
        assert_eq!(my_linked_list.get(0), -1);
    }
}

# 删除链表倒数第N个节点 - 多实现对比

## 1. 问题描述

LeetCode 19 问题要求删除链表的倒数第 n 个节点，并返回链表的头节点。

**输入**：head = [1,2,3,4,5], n = 2  
**输出**：[1,2,3,5]  

**输入**：head = [1], n = 1  
**输出**：[]  

**输入**：head = [1,2], n = 1  
**输出**：[1]

**约束条件**：

- 链表中节点的数目为 sz
- 1 <= sz <= 30
- 0 <= Node.val <= 100
- 1 <= n <= sz

**进阶要求**：能否使用一趟扫描实现？

## 2. 设计思路

### 2.1 核心问题分析

这个问题的核心是如何高效地定位链表的倒数第 n 个节点：

1. 需要找到倒数第 n 个节点的前一个节点
2. 修改前一个节点的 next 指针，跳过要删除的节点
3. 需要处理边界情况，特别是删除头节点的情况

### 2.2 实现方式概述

实现这个问题有多种方式，每种方式都有其优缺点：

1. **双指针法**：使用两个指针，让第二个指针先走 n 步，然后两个指针一起走，直到第二个指针到达末尾
2. **裸指针法**：使用原始指针直接操作内存，提高访问效率
3. **递归回溯法**：通过递归从链表末尾回溯计数，定位并删除目标节点

## 3. 实现方案

### 3.1 双指针法

```rust
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
```

**工作原理**：

- 创建一个虚拟头节点 `dummy`，避免处理删除头节点的特殊情况
- 让 `right` 指针先走 n 步
- 然后 `left` 和 `right` 指针同时移动，直到 `right` 到达链表末尾
- 使用 `unsafe` 代码将不可变引用转换为可变引用，修改 `left` 节点的 `next` 指针
- 使用 `take()` 安全地断开链表，删除目标节点

### 3.2 裸指针法

```rust
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
```

**工作原理**：

- 创建虚拟头节点 `dummy`，统一处理边界情况
- `left` 使用可变引用，`right` 使用裸指针
- `right` 指针先走 n 步，然后两个指针同时移动
- 定位到目标节点后，直接修改 `left` 节点的 `next` 指针
- 使用 `take()` 安全地删除目标节点

### 3.3 递归回溯法

```rust
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
            let index = 1 + Self::travel(next, n);
            if index == n {
                noop.next = next.next.take();
            }
            index
        }
        None => 0,
    }
}
```

**工作原理**：

- 使用 `dummy` 节点统一处理删除头节点的情况
- 递归函数 `travel` 从 `dummy` 节点开始，返回每个节点到尾节点的距离
- 递归一直走到链表末尾（返回 0），然后回溯时递增计数
- 当计数等于 n 时，当前节点的 `next` 就是要删除的节点
- 使用 `take()` 安全地断开链表，实现删除

## 4. 性能分析

### 4.1 时间复杂度

- 双指针法：O(n)，只需要一次遍历链表
- 裸指针法：O(n)，同样只需要一次遍历链表
- 递归回溯法：O(n)，递归深度为链表长度

### 4.2 空间复杂度

- 双指针法：O(1)，只使用了常数个额外变量
- 裸指针法：O(1)，只使用了常数个额外变量
- 递归回溯法：O(n)，递归调用栈的深度为链表长度

### 4.3 性能比较

| 实现方式 | 优点 | 缺点 | 适用场景 |
| --------- | ------ | ------ | --------- |
| 双指针法 | 高效，只需要一次遍历 | 使用了 unsafe 代码 | 追求代码简洁和较好性能的场景 |
| 裸指针法 | 访问效率最高 | 使用了 unsafe 代码，容易出错 | 对性能要求极高的场景 |
| 递归回溯法 | 代码逻辑清晰，安全 | 空间复杂度高 | 链表长度不大，追求代码可读性的场景 |

## 5. Rust实现特色

### 5.1 所有权管理

Rust的所有权系统确保链表操作的安全性：

- 使用 `Box<ListNode>` 管理节点的内存
- 使用 `Option` 处理空节点的情况
- 使用 `take()` 方法安全地转移节点的所有权

### 5.2 虚拟头节点

三种实现都使用了虚拟头节点（dummy）来统一处理删除头节点的边界情况，提高了代码的简洁性。

### 5.3 unsafe代码的谨慎使用

双指针法和裸指针法中使用了 `unsafe` 代码，但都非常谨慎：

- 只在必要时使用，并且有明确的注释说明
- 使用 `allow(mutable_transmutes)` 允许类型转换
- 使用 `take()` 确保内存安全

## 6. 测试设计

### 6.1 基本功能测试

```rust
// 测试删除中间节点
assert_eq!(
    Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
    to_list(vec![1, 2, 3, 5])
);
// 测试删除头节点
assert_eq!(Solution::remove_nth_from_end(to_list(vec![1]), 1), None);
```

### 6.2 边界条件测试

```rust
// 测试删除尾节点
assert_eq!(
    Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 1),
    to_list(vec![1, 2, 3, 4])
);
// 测试删除链表中唯一节点
assert_eq!(Solution::remove_nth_from_end(to_list(vec![5]), 1), None);
```

### 6.3 多实现对比测试

```rust
// 测试三种实现的结果一致性
let input = to_list(vec![1, 2, 3, 4, 5]);
let output1 = Solution::remove_nth_from_end(input.clone(), 2);
let output2 = Solution::remove_nth_from_end_raw_ptr(input.clone(), 2);
let output3 = Solution::remove_nth_from_end_travel(input.clone(), 2);
assert_eq!(output1, output2);
assert_eq!(output1, output3);
```

## 7. 扩展思考

### 7.1 其他实现方式

- **两次遍历法**：第一次遍历获取链表长度，第二次遍历定位并删除目标节点（时间复杂度O(n)，空间复杂度O(1)）
- **栈法**：使用栈存储所有节点，然后弹出n+1个节点，修改其next指针（时间复杂度O(n)，空间复杂度O(n)）

### 7.2 相关LeetCode问题

- [876. 链表的中间结点](https://leetcode.com/problems/middle-of-the-linked-list/)：使用快慢指针定位链表中间节点
- [142. 环形链表 II](https://leetcode.com/problems/linked-list-cycle-ii/)：使用快慢指针检测链表环并找到环的入口
- [206. 反转链表](https://leetcode.com/problems/reverse-linked-list/)：反转链表的基础操作

### 7.3 实际应用场景

- 链表编辑器中的删除操作
- 缓存系统中的淘汰策略
- 分布式系统中的消息队列管理

## 8. 总结

LeetCode 19问题展示了删除链表节点的经典实现方式，每种实现都有其特点：

1. **双指针法**：高效且简洁，通过两个指针的配合实现了一次遍历完成操作，是最常用的方法
2. **裸指针法**：提供了最高的访问效率，但牺牲了一定的安全性
3. **递归回溯法**：代码逻辑清晰直观，通过递归计数巧妙地定位目标节点，但空间复杂度较高

Rust的所有权系统和类型安全特性为链表操作提供了保障，同时也允许在必要时使用unsafe代码进行性能优化。三种实现方式各有优缺点，应根据具体场景选择合适的方法。

递归回溯法的核心思想是利用递归的特性从链表末尾开始计数，当计数达到n时，当前节点的next指针指向的就是要删除的倒数第n个节点。这种方法避免了复杂的指针操作，代码逻辑清晰，是解决此类问题的一种优雅思路。

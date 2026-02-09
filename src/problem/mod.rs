#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}
fn main() {
    let mut head = Some(Box::new(ListNode { val: 1, next: None }));
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut cur = &mut dummy;
    // 核心报错逻辑
    let nxt = cur.next.as_mut().unwrap(); // 可变借用 cur.next
    if true {
        cur.next = None; // ❌ 报错: cannot assign to `cur.next` because it is borrowed
    } else {
        let _ = nxt; // 使用借用
    }
    // let list = std::collections::LinkedList::new();
}

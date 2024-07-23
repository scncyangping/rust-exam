// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// 给定一个链表的头节点 head 和一个整数 val
// 请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点
// 203
pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    dummy_head.next = head;
    let mut cur = dummy_head.as_mut();

    while let Some(nxt) = cur.next.take() {
        if nxt.val == val {
            cur.next = nxt.next;
        } else {
            cur.next = Some(nxt);
            cur = cur.next.as_mut().unwrap();
        }
    }
    dummy_head.next
}

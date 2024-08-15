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

mod my_self_link {
    /**
     * Your MyLinkedList object will be instantiated and called as such:
     * let obj = MyLinkedList::new();
     * let ret_1: i32 = obj.get(index);
     * obj.add_at_head(val);
     * obj.add_at_tail(val);
     * obj.add_at_index(index, val);
     * obj.delete_at_index(index);
     */

    #[derive(Default)]
    struct ListLink {
        val: i32,
        next: Option<Box<ListLink>>,
    }

    #[derive(Default)]
    struct MyLinkedList {
        head: Option<Box<ListLink>>,
    }

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl MyLinkedList {
        fn new() -> Self {
            Default::default()
        }

        fn get(&self, index: i32) -> i32 {
            if index < 0 {
                return -1;
            }
            let mut cur = &self.head;
            let mut count = 0;
            while let Some(en) = cur {
                // 有下一个
                if count == index {
                    return en.val;
                }
                count += 1;
                cur = &en.next;
            }
            return -1;
        }

        fn add_at_head(&self, val: i32) {}

        fn add_at_tail(&self, val: i32) {}

        fn add_at_index(&self, index: i32, val: i32) {}

        fn delete_at_index(&self, index: i32) {}
    }
}

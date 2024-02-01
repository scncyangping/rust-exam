enum Link {
    Empty,
    More(Box<Node>),
}
struct Node {
    elem: i32,
    next: Link,
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        let new_code = Node {
            elem,
            // 无法移动
            // next: self.head,
            // 替换某个可变借用中的某个值为新值
            // 同时获取到其原值的所有全
            next: std::mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(new_code))
    }

    pub fn pop(&mut self) -> Option<i32> {
        // match &self.head {
        //     Link::Empty => return None,
        //     Link::More(node) => {
        //         // match &self.head 只是一个引用,node.next不能获取其所有权
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => return None,
            // 这里拿到的就是其所有权
            Link::More(node) => {
                // &self.head 只是一个引用,node.next不能获取其所有权
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut clear_link = std::mem::replace(&mut self.head, Link::Empty);
        // 手动实现drop,避免过长时,自动drop失效
        while let Link::More(mut node) = clear_link {
            // 获取到所有权
            // 作用域结束时释放
            clear_link = std::mem::replace(&mut node.next, Link::Empty)
        }
    }
}

// in first.rs
#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn long_list() {
        let mut list = List::new();
        for i in 0..100000 {
            list.push(i);
        }
        drop(list)
    }
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

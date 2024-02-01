type BN<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: BN<T>,
}

pub struct List<T> {
    head: BN<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let new_code = Node {
            elem,
            // 无法移动
            // next: self.head,
            // 替换某个可变借用中的某个值为新值
            // 同时获取到其原值的所有全
            //next: std::mem::replace(&mut self.head, None),
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_code))
    }

    pub fn pop(&mut self) -> Option<T> {
        // match &self.head {
        //     Link::Empty => return None,
        //     Link::More(node) => {
        //         // match &self.head 只是一个引用,node.next不能获取其所有权
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        // match self.head.take() {
        //     None => return None,
        //     // 这里拿到的就是其所有权
        //     Some(node) => {
        //         // &self.head 只是一个引用,node.next不能获取其所有权
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut clear_link = self.head.take();
        // 手动实现drop,避免过长时,自动drop失效
        while let Some(mut node) = clear_link {
            // 获取到所有权
            // 作用域结束时释放
            clear_link = node.next.take()
        }
    }
}

struct IntoIter<T>(List<T>);

impl<T> List<T> {
    fn into_iter(self) -> IntoIter<T> {
        IntoIter::<T>(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            // 方法1
            //next: self.head.as_ref().map(|node| &**node),
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            //self.next = node.next.as_ref().map(|node| &**node);
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iterm_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iterm_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
    #[test]
    fn long_list() {
        let mut list = List::new();
        for i in 0..1000000 {
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
        list.push("1");
        list.push("2");
        list.push("3");

        // Check normal removal
        assert_eq!(list.pop(), Some("3"));
        assert_eq!(list.pop(), Some("2"));

        // Push some more just to make sure nothing's corrupted
        list.push("4");
        list.push("5");

        // Check normal removal
        assert_eq!(list.pop(), Some("5"));
        assert_eq!(list.pop(), Some("4"));

        // Check exhaustion
        assert_eq!(list.pop(), Some("1"));
        assert_eq!(list.pop(), None);
    }
}

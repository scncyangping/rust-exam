use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node(usize, Option<Rc<RefCell<Node>>>);

impl Node {
    fn new(id: usize) -> Node {
        Node(id, None)
    }
    fn set_down_stream(&mut self, stream: Rc<RefCell<Node>>) {
        self.1 = Some(stream);
    }

    fn get_down_stream(&self) -> Option<Rc<RefCell<Node>>> {
        self.1.as_ref().map(|v| v.clone())
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);

    let node4 = Node::new(4);

    node3.set_down_stream(Rc::new(RefCell::new(node4)));

    node1.set_down_stream(Rc::new(RefCell::new(node3)));

    node2.set_down_stream(node1.get_down_stream().unwrap());

    println!("node1: {:?} node2: {:?}", node1, node2);

    let node5 = Node::new(5);

    let node3 = node1.get_down_stream().unwrap();

    node3
        .borrow_mut()
        .set_down_stream(Rc::new(RefCell::new(node5)));

    println!("node1: {:?} node2: {:?}", node1, node2);
}


// Rc<T> 允许相同数据有多个所有者，Box<T>和RefCell<T>有单一所有者
// Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查
// RefCell<T> 允许在运行时执行不可变或可变借用检查


// 因为RefCell<T>允许在运行时执行可变借用检查，所以我们可以在即便RefCell<T>自身是不可变的情况下修改其内部的值


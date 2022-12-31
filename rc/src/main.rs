use std::rc::Rc;

#[derive(Debug)]
struct Node(usize, Option<Rc<Node>>);

impl Node {
    fn new(id: usize) -> Node {
        Node(id, None)
    }
    fn set_down_stream(&mut self, stream: Rc<Node>) {
        self.1 = Some(stream);
    }

    fn get_down_stream(&self) -> Option<Rc<Node>> {
        self.1.as_ref().map(|v| v.clone())
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);

    let node4 = Node::new(4);

    node3.set_down_stream(Rc::new(node4));

    node1.set_down_stream(Rc::new(node3));

    node2.set_down_stream(node1.get_down_stream().unwrap());

    print!("node1: {:?} node2: {:?}", node1, node2);
}

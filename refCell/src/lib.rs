mod mss {
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        msg: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(msg: &T, max: usize) -> LimitTracker<T> {
            LimitTracker { msg, value: 0, max }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;
            let percentage_of_max = self.value as f64 / self.max as f64;
            if percentage_of_max >= 1.0 {
                self.msg.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.msg
                    .send("Urgent warning: You've used up over 90% of your qu");
            } else if percentage_of_max >= 0.75 {
                self.msg
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }
}

mod weak {
    use std::{
        cell::RefCell,
        rc::{Rc, Weak},
    };

    #[derive(Debug)]
    pub struct Node {
        pub value: i32,
        pub parent: RefCell<Weak<Node>>,
        pub children: RefCell<Vec<Rc<Node>>>,
    }
}

mod tests {
    use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::rc::Weak;

    use super::mss::*;
    use super::weak::*;

    #[test]
    fn test_weak() {
        // 有父子节点层级关系，父节点引用子节点，子节点也有父节点引用
        // 父节点包含子节点，但是子节点不应该拥有父节点
        // 使用Weak引用来表示这种关系 Rc::downgrade
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            // RefCell<Vec<Rc<Node>>>,
            let branch = Rc::new(Node {
                value: 4,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });
            // 获取父节点的引用
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow_mut().upgrade());

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    struct MockMessenger {
        //send_message: Vec<String>,
        send_message: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_message: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            //self.send_message.push(String::from(msg));
            self.send_message.borrow_mut().push(String::from(msg));

            // 只要发生了多次borrow_mut()就会panic,不论生命周期
            //
            // let mut one_borrow = self.send_message.borrow_mut();
            // one_borrow.push(String::from(msg));
            // {
            //     let mut two_borrow = self.send_message.borrow_mut();
            //     two_borrow.push(String::from(msg));
            // }
        }
    }

    #[test]
    fn test_75() {
        let mes = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mes, 100);
        limit_tracker.set_value(80);
        for ele in mes.send_message.borrow().iter() {
            println!("{}", ele)
        }
        //assert_eq!(mes.send_message.len(), 1)
        assert_eq!(mes.send_message.borrow().len(), 1);
    }
}

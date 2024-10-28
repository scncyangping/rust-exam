use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use crate::List::{Cons, Nil};
    use std::{rc::Rc, sync::Mutex};

    #[test]
    fn test_strong_count() {
        let x = 2;
        match x {
            2 => panic!(""),
            _ => println!("123"),
        }
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let _b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let _c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    #[test]
    fn test() {
        // let ve = vec![1, 2, 3, 4];
        // for (i, v) in ve.iter().enumerate() {
        //     println!("i = {}, v = {}", i, v);
        // }
        // println!("ve = {:?}", ve);
        // let x: Option<u32> = None;
        // if let Some(a) = x {
        //     println!("a = {:?}", a);
        // } else {
        //     println!("123")
        // }

        let mut vector = vec![1, 2, 3, 4, 5, 6];
        let (left, right) = split_at_mut(&mut vector, 3);

        println!("left = {:?} right = {:?}", left, right);
    }

    use std::slice;
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();
        assert!(mid <= len);
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
}

// mod ttt;

// fn Test() {
//     ttt::TTT();
// }

use std::fmt::Display;

pub trait A {
    fn test();
}
impl<T: Display> A for T {
    fn test() {
        todo!()
    }
}

// 闭包是可以保存进变量或作为参数传递给其他函数的匿名函数
// 可以在一个地方创建闭包，然后在不同的上下文中执行闭包运算
// 闭包允许捕获调用者作用域的值

use std::env;

// 闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式:获取所有权，可变借用 和不可变借用。这三种捕获值的方式被编码为如下三个 Fn trait:
// • FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消 费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。
//          其名称的 Once 部分代 表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
// • FnMut 获取可变的借用值所以可以改变其环境
// • Fn 从其环境获取不可变的借用值
// struct Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     calculation: T,
//     value: Option<u32>,
// }
//
// impl<T> Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher {
//             calculation: calculation,
//             value: None,
//         }
//     }
//
//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

fn main() {
    let x = Box::new(String::from("value"));
    pri(&x);
    drop(x);
    let mut name = String::from("hello");
    let mut name1 = String::from("hola");
    // 捕获 &mut name
    // 这里的闭包使用了name的可变借用
    let mut c = || {
        name.push_str("Tyr");
        println!("c {}", name);
    };
    c();
    call_mut(&mut c);
    call_once(c);
    let mut c1 = move || {
        name1.push_str("!");
        println!("c1 : {}", name1);
    };
    c1();
    call_once(c1);
    let name2 = "c2";
    let c2 = || {
        println!("c2: {}", name2);
    };
    c2();
    call_once(c2);

    let c3 = |num: u32| num * 2;

    call_once_num(12, c3);

    let mut cache = Cache::new(|num| num * 2);
    let mut cache = Cache::new(|num| num * 21);
    println!("{}", cache.execute(1));
    println!("{}", cache.execute(2));
    println!("{}", cache.execute(3));
    let mut cache = Cache::new(|num| num * 21);
    println!("{}", cache.execute(3));
}

// 在作为参数时，FnMut也要显示地使用 mut 或者 &mut
fn call_mut(c: &mut impl FnMut()) {
    c();
}

fn call_once(c: impl FnOnce()) {
    c();
}

fn call_once_num(num: u32, c: impl FnOnce(u32) -> u32) {
    c(num);
}

fn pri(name: &str) {
    println!("{}", name);
}

// 定义一个缓存器,每次调用增加指定步长

struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    value: Option<u32>,
    func: T,
}

// 定义方法
impl<T> Cache<T>
where
    T: Fn(u32) -> u32,
{
    fn new(f: T) -> Self {
        Cache {
            value: None,
            func: f,
        }
    }

    fn execute(&mut self, num: u32) -> u32 {
        match self.value {
            None => {
                self.value = Some(num);
                num
            }
            Some(a) => {
                let rs = a + (&self.func)(num);
                self.value = Some(rs);
                rs
            }
        }
    }
}

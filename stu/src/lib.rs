pub mod cli_input;

#[cfg(test)]
mod tests {
    use std::{future, vec};
    #[derive(Debug)]
    enum MyEnum {
        Foo,
        Bar(u32),
        Dar(String),
        Ear(u32, String),
        Far { A: u32, B: String },
    }
    #[test]
    fn test_fileter_enum() {
        let v = vec![MyEnum::Foo, MyEnum::Bar(1)];
        let x: Vec<MyEnum> = v
            .into_iter()
            // 直接 == 判断会报错
            //.filter(|x| x == MyEnum::Foo)
            .filter(|x| matches!(x, MyEnum::Foo))
            .collect();
    }
    struct TestMatchPartten {
        A: String,
        B: String,
    }
    /// 模式匹配
    #[test]
    fn test_match_pattern() {
        let x: TestMatchPartten = TestMatchPartten {
            A: String::from("value is a"),
            B: String::from("value is b"),
        };
        let TestMatchPartten { A: c, B: d } = x;
        // 模式匹配会移除所有权
        // println!("a is {},b is {}", x.A, x.B);
        println!("a is {},b is {}", c, d);

        let enum1 = MyEnum::Far {
            A: 12,
            B: String::from("123"),
        };

        let enum2 = MyEnum::Dar("value".to_string());
        let enum3 = MyEnum::Bar(32);
        let zz = 30;
        match enum3 {
            // 直接使用 _ 就不会匹配
            // MyEnum::Dar(_) => println!("match dar1"),
            // _s 会忽略这个未使用的提示,但是还是会赋值
            MyEnum::Dar(_s) => println!("match dar2"),
            MyEnum::Ear(x, y) => println!("ear {}: {}", x, y),
            //  变量绑定,必须有 = 符号
            MyEnum::Far {
                A: x @ 1..=10,
                B: b,
            } => println!("ear {}: {}", x, b),
            // 可以使用 .. 符号来忽略模式匹配中其他的值
            MyEnum::Far { A: a, .. } => println!("far {}: {}", a, a),
            // 匹配守卫: 可以在匹配中增加判断,匹配守卫可以使用外部的变量
            MyEnum::Bar(z) if z < zz => println!("{}", z),
            MyEnum::Bar(x) => println!("x {}", x),
            _ => println!("unknown"),
        }
    }

    #[test]
    fn test_bind() {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => {
                println!("Found an id in range: {}", id_variable)
            }
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => {
                println!("Found some other id: {}", id)
            }
        }
    }
    #[test]
    fn test_find() {
        let x: [u32; 4] = [1, 2, 3, 4];
        let xx: [i32; 4] = [1, 2, 3, 4];
        let xxx: [f32; 4] = [11.1, 12.2, 1.3, 4 as f32];
        dbg!(find_max_num(&x));
        dbg!(find_max_num(&xx));

        let v1 = vec![12];
        let v2 = vec![12];
        let x = &v2[0];
        let xx = v2.get(0);
        let xxx = xx.copied();

        let y = vec![String::from("123")];
        // copied方法仅能应用到copy类型
        //let yy = y.get(0).copied();
    }
    fn find_max_num<T>(s: &[T]) -> &T
    where
        T: Ord,
    {
        s.iter().max().take().unwrap()
    }
}

pub trait traitA<T: Default, R = u8, D = u8>: traitB {
    type Item;
    fn add(&self, r: R, i: Self::Item, t: T) -> Self;
}

pub trait traitB {
    fn traitB();
}

pub struct StructC<T>
where
    T: Default,
{
    name: T,
}

impl<T> traitA<T, u128> for StructC<T>
where
    T: Default,
{
    type Item = u16;

    fn add(&self, r: u128, i: Self::Item, t: T) -> Self {
        todo!()
    }
}

impl<T> traitA<T, usize> for StructC<T>
where
    T: Default,
{
    type Item = u16;

    fn add(&self, r: usize, i: Self::Item, t: T) -> Self {
        todo!()
    }
}

impl<T> traitA<usize> for StructC<T>
where
    T: Default,
{
    type Item = u16;

    fn add(&self, r: u8, i: Self::Item, t: usize) -> Self {
        todo!()
    }
}

impl<T> traitB for StructC<T>
where
    T: Default,
{
    fn traitB() {
        println!("structC for traitB")
    }
}

pub trait traitC {
    type Item;
    fn test(i: Self::Item);
}

struct StructB {}

impl traitC for StructB {
    type Item = u32;

    fn test(i: u32) {
        todo!()
    }
}
// 实现一样的只有item不一样是不可以的
// impl traitC for StructB {
//     type Item=u8;

//     fn test(i: u8) {
//         todo!()
//     }
// }

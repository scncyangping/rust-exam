pub mod cli_input;

#[cfg(test)]
mod tests {
    use std::vec;

    enum MyEnum {
        Foo,
        Bar,
    }
    #[test]
    fn test_fileter_enum() {
        let v = vec![MyEnum::Foo, MyEnum::Bar];
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
    }
}

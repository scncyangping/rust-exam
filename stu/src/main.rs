use std::fmt::Debug;

fn print_it2<T: Debug>(input: &'static T) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}
#[derive(Debug)]
struct TempRef<'a> {
    temp: &'a String,
}

fn main() {
    let temp = String::from("temporary");
    let temp_ref = TempRef { temp: &temp };
    // temp_ref 不满足 'static 约束，因为它包含对局部 String 的引用
    // 这行会导致编译错误
    // 编译器会校验,传入的类型temp是否满足‘static。因为它有自己的生命
    // 周期参数‘a，所以不是‘static。
    //print_it(&temp_ref); 
    static xxx: &str = "123";
    print_it2(&xxx);
}

fn main() {
    let x = 0;
    let sum = self::add!(1, 2);
    println!("sum: {}", sum);
    let sum = self::add!(x);
    println!("sum: {}", sum);
    let sum = self::add_more!(1, 2, 3, 4);
    println!("sim: {}", sum);
}
// 常见的匹配方式有 7 种
// expr：匹配表达式
// ty：匹配类型
// stmt：匹配语句
// item：匹配一个 item
// ident：匹配一个标识符
// path：匹配一个 path
// tt：匹配一个 token tree

// 常见的重复符号有 3 个。
// * 表示重复 0 到多次。
// + 表示重复 1 到多次。
// ? 表示重复 0 次或 1 次。

#[macro_export]
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
    ($a:expr) => {
        $a
    };
}

#[macro_export]
macro_rules! add_more {
    ($($a:expr),*) => {
        0
        // 重复添加
        $(+ $a)*
    };
}

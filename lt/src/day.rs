#[cfg(test)]
mod tests {

    #[test]
    fn test_return_type() {
        fn result_with_error(i: u32) -> Result<u32, Box<dyn std::error::Error>> {
            if i > 10 {
                Err("more than 10".to_string())?
            }
            Ok(12)
        }

        match result_with_error(12) {
            Ok(o) => println!("ok with {o}"),
            Err(e) => println!("err with {e}"),
        }
    }
    #[test]
    /// ? 操作符运用
    fn test_in_link() {
        fn process_result(res: Result<u8, &str>) -> Result<u8, &str> {
            res.map(|x| {
                let y = x.checked_add(1).ok_or("Overflow1 occurred")?; // 使用 ? 操作符简化错误处理
                y.checked_mul(2).ok_or("Overflow2 occurred") // 检查乘法是否溢出
            })?
        }

        let res = Ok(255);
        match process_result(res) {
            Ok(result) => println!("Processed result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}

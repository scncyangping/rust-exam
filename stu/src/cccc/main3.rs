fn example_function(input: i32) -> String {
    if input < 32 {
        match input {
            1 => return "one".to_string(),
            2 => return "two".to_string(),
            3 => return "three".to_string(),
            _ => return "other".to_string(),
        }
    } else {
        return "111".to_string();
    }
}

fn main() {
    let value = example_function(35);
    println!("{}", value); // 输出: two
}

//! This function  is used to test that the user cli input

use std::io;

fn user_input() {
    let a = [1, 2, 3, 4, 5];
    let mut index = String::new();

    io::stdin().read_line(&mut index).unwrap();

    let x = index.trim().parse::<usize>().unwrap();
    let elment = a[x];
    println!("{}", elment);
}
#[cfg(test)]
mod tests {
    use super::user_input;

    #[test]
    fn test_user_input() {
        let a: [i32; 5] = [1, 2, 3, 4, 5];

        let slice: &[i32] = &a[1..3];

        assert_eq!(slice, &[2, 3]);

        let array: [String; 3] = std::array::from_fn(|_i| String::from("value"));
        println!("{:?}", array);
        user_input()
    }
    /// loop 方法可以break带上值
    ///
    #[test]
    fn test_for_while_loop() {
        let i = 1;
        let x = loop {
            if i == 1 {
                break i;
            }
        };
        dbg!(x);
    }
}

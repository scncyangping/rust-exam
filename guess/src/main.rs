use std::io;

use rand::Rng;

fn main() {
    println!("welcome guess game! please enter a number");
    let old_number = rand::thread_rng().gen_range(1..101);
    println!("old_number: {}", old_number);

    loop {
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("read_line failed");

        let number: i32 = number.trim().parse().expect("trim failed");

        println!("number: {}", number);
        // if old_number > number {
        //     println!("too small");
        // } else if old_number < number {
        //     println!("too max");
        // } else {
        //     print!("you win");
        //     break;
        // }
        match number.cmp(&old_number) {
            std::cmp::Ordering::Less => println!("to small"),
            std::cmp::Ordering::Equal => {
                println!("you win");
                break;
            }
            std::cmp::Ordering::Greater => println!("too big"),
        }
    }
}

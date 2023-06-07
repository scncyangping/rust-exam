fn main() {
    let s = String::from("hello world");
    // let x = 1;
    // match x {
    //     1 => println!("x = 1"),
    //     2 | 3 => println!("x = 5"),
    //     1..=5 => println!("x = 5"),
    //     _ => println!("x = {}", x),
    // }

    // struct Point {
    //     x: i32,
    //     y: i32,
    // }

    // let p = Point { x: 0, y: 7};
    // let Point { x, y} = p;

    // assert_eq!(0,x);
    // assert_eq!(7,y);

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("x < 5"),
        Some(x) => println!("x = {}", x),
        None => (),
    }

    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_va @ 3..=7 } => println!("Hello id_va: {}", id_va),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in anather range")
        }
        Message::Hello { id } => println!("Hello: {}", id),
    }
}

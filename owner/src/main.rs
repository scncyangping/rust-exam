fn main() {
    // let data = vec![10, 42, 9, 8];

    // let data1 = &data;

    // println!("addr of value : {:p}({:p}), addr of data {:p}, data1: {:p}", &data, data1, &&data, &data1);

    // let v = 42;

    // if let Some(post) = find_pos(data, v) {
    //     println!("Found {} at {}", v, post)
    // }

    let data = vec![10, 42, 9, 8];

    let data1 = &data;

    println!(
        "addr of value : {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );

    println!(
        "addr of items : [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
    println!("sum of data1: {}", sum(&data));

    println!(
        "addr of items : [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    )
}

fn sum(data: &Vec<u32>) -> u32 {
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(1, |acc, x| acc + x)
}

fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }
    None
}

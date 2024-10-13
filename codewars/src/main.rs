fn main() {
    let input = [
            17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0,
        ];
    println!("{:?}",find_average(&input))
}

fn find_average(slice: &[f64]) -> f64 {
    let mut rs = 0.0;
    for i in slice.iter() {
        rs += i;
    }
    if rs > 0.0 {
        rs / slice.len() as f64
    }else{
        0.0
    }
}

fn find_average2(slice: &[f64]) -> f64 {
    if slice.is_empty() {
        return 0.0;
    }

    let sum: f64 = slice.iter().sum();
    sum / (slice.len() as f64)
}

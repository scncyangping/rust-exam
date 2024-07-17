// [6,7,8,10,14,15] 15
//
fn two_split_search(arr: &[i32], target: i32) -> usize {
    let index = arr.len() / 2;
    let (mut start, mut end) = (0, 0);
    let value = arr[index];
    if value == target {
        return index;
    } else if value > target {
        start = 0;
        end = index
    } else if value < target {
        start = index;
        end = arr.len()
    }
    return two_split_search(&arr[start..end], target) + start;
}

#[cfg(test)]
mod tests {
    use super::two_split_search;

    #[test]
    fn test_two_split() {
        let x = vec![-1, 0, 3, 5, 9, 12];
        let index = two_split_search(&x, 12);
        println!("index: {index}")
    }
}

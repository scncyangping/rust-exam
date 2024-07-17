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

fn two_split_search_v2(arr: &[i32], target: i32) -> i32 {
    // 左闭右开
    let (mut left, mut right) = (0, arr.len());
    while left < right {
        let index = (left + right) / 2;
        // 左闭右开
        match arr[index].cmp(&target) {
            std::cmp::Ordering::Less => left = index + 1,
            std::cmp::Ordering::Equal => return index as i32,
            std::cmp::Ordering::Greater => right = index,
        }
    }
    -11
}

/// 左闭右闭
fn two_split_search_v3(arr: &[i32], target: i32) -> i32 {
    // 左闭右开
    let (mut left, mut right) = (0, arr.len() as i32 - 1);
    while left <= right {
        let index = (left + right) / 2;
        // 左闭右开
        match arr[index as usize].cmp(&target) {
            std::cmp::Ordering::Less => left = index + 1,
            std::cmp::Ordering::Equal => return index as i32,
            std::cmp::Ordering::Greater => right = index - 1,
        }
    }
    -11
}

#[cfg(test)]
mod tests {
    use crate::array::{two_split_search_v2, two_split_search_v3};

    use super::two_split_search;

    #[test]
    fn test_two_split() {
        let x = vec![-1, 0, 3, 5, 9, 12];
        let index = two_split_search(&x, 12);
        println!("index: {index}")
    }
    #[test]
    fn test_two_split_v2() {
        let x = vec![-2, 0, 3, 5, 12];
        let index = two_split_search_v2(&x, 12);
        println!("index: {index}")
    }

    #[test]
    fn test_two_split_v3() {
        let x = vec![-2, 0, 3, 5, 12,13];
        let index = two_split_search_v3(&x, -1);
        println!("index: {index}")
    }
}

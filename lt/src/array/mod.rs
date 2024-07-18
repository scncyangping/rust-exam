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

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len() as i32);
    while left < right {
        let mid = (left + right) / 2;
        match nums[mid as usize].cmp(&target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Equal => return mid,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    -1
}

// 给定一个数组 nums 和一个值 val，你需要 原地 移除所有数值等于 val 的元素，并返回移除后数组的新长度。
// 不要使用额外的数组空间，你必须仅使用 O(1) 额外空间并原地修改输入数组。
// 元素的顺序可以改变。你不需要考虑数组中超出新长度后面的元素。
// 示例 1: 给定 nums = [3,2,2,3], val = 3, 函数应该返回新的长度 2, 并且 nums 中的前两个元素均为 2。 你不需要考虑数组中超出新长度后面的元素。
// 示例 2: 给定 nums = [0,1,2,2,3,0,4,2], val = 2, 函数应该返回新的长度 5, 并且 nums 中的前五个元素为 0, 1, 3, 0, 4。
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut index = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[index] = nums[i];
            index += 1;
        } else {
            continue;
        }
    }
    index as i32
}

#[cfg(test)]
mod tests {
    use crate::array::{remove_element, two_split_search_v2, two_split_search_v3};

    use super::two_split_search;
    #[test]
    fn test_remove_element() {
        let mut x = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let index = remove_element(&mut x, 2);
        println!("index: {index}\n vec: {:?}", x)
    }
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
        let x = vec![-2, 0, 3, 5, 12, 13];
        let index = two_split_search_v3(&x, -1);
        println!("index: {index}")
    }
}

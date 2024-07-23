use std::vec;

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

// 给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方 组成的新数组，要求也按 非递减顺序 排序。
// 输入：nums = [-4,-1,0,3,10]
// 输出：[0,1,9,16,100]
// 解释：平方后，数组变为 [16,1,0,9,100]，排序后，数组变为 [0,1,9,16,100]
// 977
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    // 非递减顺序,说明最左边为最小,最右边为最大,需要注意的是
    // 负数平方过后可能会比正数还大
    // 分析可以,若以0为区分,则,正数部分最右边平方也为最大
    // 负数部分,最左侧平方最大,所以采用双指针法
    // 分别从左边和右边移动,若确定为大于另一方,则将当前方向指针移动到下一个
    // 当左边指针小于等于右边指针,继续移动
    let n = nums.len();
    let (mut i, mut j, mut k) = (0, n - 1, n);
    let mut ans = vec![0; n];
    while i <= j {
        if nums[i] * nums[i] < nums[j] * nums[j] {
            ans[k - 1] = nums[j] * nums[j];
            j -= 1;
        } else {
            ans[k - 1] = nums[i] * nums[i];
            i += 1;
        }
        k -= 1;
    }
    ans
}

// 给定一个含有 n 个正整数的数组和一个正整数 s ，找出该数组中满足其和 ≥ s 的长度最小的 连续 子数组，并返回其长度。如果不存在符合条件的子数组，返回 0。
// 示例：
// 输入：s = 7, nums = [2,3,1,2,4,3]
// 输出：2
// 解释：子数组 [4,3] 是该条件下的长度最小的子数组
// 209
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    // 滑动窗口
    // 使用双指针表示滑动窗口
    // index表示窗口结束位置,start_index表示窗口开始位置
    let (mut sum, mut start_inex, mut result) = (0, 0, i32::MAX);

    for (index, val) in nums.iter().enumerate() {
        sum += val;
        // 寻找大于等于
        while sum >= target {
            let sub_length = (index - start_inex + 1) as i32;
            if result > sub_length {
                result = sub_length;
            }
            sum -= nums[start_inex];
            start_inex += 1;
        }
    }

    if result == i32::MAX {
        return 0;
    }
    result
}
#[cfg(test)]
mod tests {
    use crate::array::{
        min_sub_array_len, remove_element, two_split_search_v2, two_split_search_v3,
    };

    use super::{sorted_squares, two_split_search};

    #[test]
    fn test_min_sub_array_len() {
        let x = vec![2, 3, 1, 2, 4, 3];
        let res = min_sub_array_len(11, x);
        println!("vec: {:?}", res)
    }

    #[test]
    fn test_sorted_squares() {
        let x = vec![1];
        let res = sorted_squares(x);
        println!("vec: {:?}", res)
    }
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

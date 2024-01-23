use std::collections::HashMap;
fn main() {
    let rs = remove_element(&mut vec![2, 1, 3, 3], 2);
    println!("{:?}", rs);
}
// 027移除元素
// 给你一个数组 nums 和一个值 val，你需要 原地
// 移除所有数值等于 val 的元素，并返回移除后数组的新长度
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut index = 0;
    for pos in 0..nums.len() {
        if nums[pos] != val {
            nums[index] = nums[pos];
            index += 1;
        }
    }
    return index as i32;
    // 待插入位置索引
    // let mut index = 0;
    // let mut p = 0;
    // while p < nums.len() {
    //     if nums[p] == val {
    //         p += 1;
    //         continue;
    //     } else {
    //         nums[index] = nums[p];
    //         index += 1
    //     }
    //     p += 1;
    // }
    // index as i32
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m as usize;
    let mut n = n as usize;
    let mut right = m + n;

    while n > 0 {
        right -= 1;
        if m == 0 || nums1[m - 1] < nums2[n - 1] {
            nums1[right] = nums2[n - 1];
            if n > 0 {
                n -= 1
            }
        } else {
            nums1.swap(m - 1, right);
            if m > 0 {
                m -= 1
            }
        }
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (k, v) in nums.into_iter().enumerate() {
        let index = target - v;
        if let Some(t) = map.get(&index) {
            return vec![k as i32, t.to_owned()];
        } else {
            map.insert(v, k as i32);
        }
    }
    vec![]
}

use std::collections::HashMap;

fn main() {
    let rs = two_sum(vec![3, 3], 6);
    println!("{:?}", rs);

    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
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

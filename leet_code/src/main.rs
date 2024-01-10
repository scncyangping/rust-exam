use std::collections::HashMap;

fn main() {
    let rs = two_sum(vec![3, 3], 6);
    println!("{:?}", rs);
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

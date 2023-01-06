use std::collections::HashMap;

fn main() {
    // capacity 增长规则: 2的幂减1  最小为3

    let mut map = HashMap::new();
    map.insert('a', 1);
    explain("added 1", &map);
    map.insert('b', 2);
    explain("added 2", &map);

    map.insert('c', 3);
    explain("added 3", &map);

    map.insert('d', 4);
    explain("added 4", &map);
    // get 参数需要使用引用 并且也返回引用
    assert_eq!(map.get(&'a'), Some(&1));
    assert_eq!(map.get_key_value(&'a'), Some((&'a', &1)));

    map.remove(&'a');

    assert_eq!(map.contains_key(&'a'), false);
    assert_eq!(map.get(&'a'), None);

    explain("remove", &map);
    // shring 后哈希表变小
    map.shrink_to_fit();
    explain("shrink_to_fit", &map);
}

fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
    println!("{}: {} {}", name, map.len(), map.capacity())
}

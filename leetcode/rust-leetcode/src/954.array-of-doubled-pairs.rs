use std::collections::HashMap;
impl Solution {
    pub fn can_reorder_doubled(mut arr: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for &n in arr.iter() {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut keys = map.keys().into_iter().cloned().collect::<Vec<i32>>();
        keys.sort_by_key(|&n| n.abs());
        for k in keys.into_iter() {
            let c1 = *map.get(&k).unwrap();
            let c2 = *map.get(&(2*k)).unwrap_or(&0);
            if c2 < c1 {
                return false;
            }
            map.insert((2*k), c2-c1);
        }
        true
    }
}


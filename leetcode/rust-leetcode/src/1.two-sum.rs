use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, n) in nums.into_iter().enumerate() {
            if map.contains_key(&n) {
                return vec![i as i32, *map.get(&n).unwrap()];
            } else {
                map.insert((target-n), i as i32);
            }
        }
        vec![]
    }
}

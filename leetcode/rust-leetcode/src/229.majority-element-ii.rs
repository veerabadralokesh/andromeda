use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for n in nums.iter() {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut ans = Vec::with_capacity(3);
        let nb3 = nums.len() / 3;
        for (k, v) in map.into_iter() {
            if v > nb3 {
                ans.push(*k);
            }
        }
        ans
    }
}


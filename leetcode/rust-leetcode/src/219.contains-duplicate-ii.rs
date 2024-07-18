impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = std::collections::HashMap::new();
        let k = k as usize;
        for i in 0..nums.len() {
            if map.contains_key(&nums[i]) && i - map.get(&nums[i]).unwrap() <= k {
                return true;
            }
            map.insert(nums[i], i);
        }
        false
    }
}


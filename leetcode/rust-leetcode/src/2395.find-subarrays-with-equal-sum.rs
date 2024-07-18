use std::collections::HashMap;
impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for i in 1..nums.len() {
            let mut count = map.entry(nums[i]+nums[i-1]).or_insert(0);
            if *count == 1 {return true;}
            *count += 1;
        }
        false
    }
}


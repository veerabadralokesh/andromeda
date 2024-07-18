use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        let mut prefix_sum = 0;
        let mut ans = 0;
        map.insert(0, 1);
        for n in nums {
            prefix_sum += n;
            ans += *map.get(&(prefix_sum - k)).unwrap_or(&0);
            *map.entry(prefix_sum).or_insert(0) += 1;
        }
        ans
    }
}


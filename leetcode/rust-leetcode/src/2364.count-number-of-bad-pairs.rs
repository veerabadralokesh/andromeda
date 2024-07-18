use std::collections::HashMap;
impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let n = nums.len() as i64;
        if n == 1 {
            return 0;
        }
        let mut ans = n * (n-1);
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            *map.entry((nums[i] - i as i32)).or_insert(0i64) += 1;
        }
        for (k, v) in map.into_iter() {
            ans -= v * (v-1);
        }
        ans/2
    }
}

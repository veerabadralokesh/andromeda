impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut p:i32 = 0;
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                p = nums[i] * nums[j];
                *map.entry(p).or_insert(0) += 1;
            }
        }
        let mut ans = 0;
        for (k, v) in map.into_iter() {
            if v > 1 {
                ans += 4 * (v - 1) * v;
            }
        }
        ans
    }
}


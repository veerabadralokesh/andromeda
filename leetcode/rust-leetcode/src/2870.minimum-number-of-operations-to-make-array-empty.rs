impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for &n in nums.iter() {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut op = 0;
        for (k, v) in map.into_iter() {
            if v == 1 {
                return -1;
            }
            match v % 3 {
                0 => op += v / 3,
                _ => op += (v / 3 + 1),
            }
        }
        op
    }
}


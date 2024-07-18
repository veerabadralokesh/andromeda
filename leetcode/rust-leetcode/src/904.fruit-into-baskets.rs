use std::collections::HashMap;
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        if fruits.len() < 3 {
            return fruits.len() as i32;
        }
        let mut map = HashMap::with_capacity(3);
        let mut ans = 0;
        let mut l = 0;
        for r in 0..fruits.len() {
            *map.entry(fruits[r]).or_insert(0) += 1;
            while l < r && map.len() > 2 {
                let mut c = map.entry(fruits[l]).or_insert(1);
                *c -= 1;
                if *c == 0 {
                    map.remove(&fruits[l]);
                }
                l += 1;
            }
            ans = ans.max(r - l + 1);
        }
        ans as _
    }
}


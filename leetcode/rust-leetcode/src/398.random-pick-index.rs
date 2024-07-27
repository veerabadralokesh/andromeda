use std::collections::HashMap;
use rand::Rng;
struct Solution {
    map: HashMap<i32, Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        let mut map = HashMap::with_capacity(nums.len());
        for (i, n) in nums.into_iter().enumerate() {
            map.entry(n).or_insert(Vec::new()).push(i as i32);
        }
        Self {map}
    }
    
    fn pick(&self, target: i32) -> i32 {
        match self.map.get(&target) {
            None => -1,
            Some(v) => {
                let idx = rand::thread_rng().gen_range(0..v.len());
                v[idx]
            }
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */


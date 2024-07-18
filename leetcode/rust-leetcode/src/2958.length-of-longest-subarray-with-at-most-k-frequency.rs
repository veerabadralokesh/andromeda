use std::collections::HashMap;
impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let mut counts = HashMap::new();
        let mut l = 0usize;
        let mut r = 0usize;
        let mut ml = 0usize;
        let mut flag = false;
        while r < nums.len() {
            let nr = nums[r];
            {
                let count = counts.entry(nr).or_insert(0);
                *count += 1;
                if *count > k {
                    flag = true;
                    *count -= 1;
                }
            }
            ml = ml.max((r - l));
            r += 1;
            if flag {
                // println!("{l} {r}");
                while nums[l] != nr {
                    *counts.get_mut(&nums[l]).unwrap() -= 1;
                    l += 1
                }
                l += 1;
                flag = false;
                // println!("{l} {r}");
            }
        }
        ml = ml.max((r - l));
        ml as i32
    }
}

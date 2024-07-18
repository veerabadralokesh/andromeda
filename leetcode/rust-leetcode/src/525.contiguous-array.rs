impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count_indices = vec![i32::MIN; 2*n+1];
        count_indices[n] = -1i32;
        let mut max_length = 0i32;
        let mut count = 0;
        for i in 0..n {
            if nums[i] == 1 {
                count += 1;
            } else {
                count -= 1;
            }
            let idx = n + (count as usize);
            if count_indices[idx] != i32::MIN {
                max_length = max_length.max(i as i32 - count_indices[idx]);
            } else {
                count_indices[idx] = i as i32;
            }
        }
        max_length as i32
    }
}

/* */

use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let (mut map, mut curr, mut ans): (HashMap<i32, i32>, i32, i32) = (HashMap::new(), 0, 0);
        map.insert(0, -1);
        for (i, num) in nums.iter().enumerate() {
            curr += { if num == &0 { -1 } else { 1 } };
            if let Some(count) = map.get(&curr) {
                ans = ans.max(i as i32 - count);
            } else {
                map.insert(curr, i as i32);
            }
        }
        ans
    }
}

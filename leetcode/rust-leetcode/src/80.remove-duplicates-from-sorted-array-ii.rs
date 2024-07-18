impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {return 1};
        let mut lp:usize = 1;
        let mut i:usize = 2;
        while i < nums.len() {
            if nums[i] == nums[lp] {
                if (nums[lp] != nums[lp-1]) {
                    lp += 1;
                    nums[lp] = nums[i];
                }
            } else {
                lp += 1;
                nums[lp] = nums[i];
            }
            i += 1;
        }
        (lp+1) as i32
    }
}

use std::collections::HashMap;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {return 1};
        let mut lp:usize = 0;
        let mut hm:HashMap<i32,i32> = HashMap::new();
        for i in 0..nums.len() {
            match hm.get_mut(&nums[i]) {
                Some(count) if *count >= 2 => continue,
                _ => {
                    let count = hm.entry(nums[i]).or_insert(0);
                    *count += 1;
                    nums[lp] = nums[i];
                    lp += 1;
                }
            }
        }
        return lp as i32;
    }
}
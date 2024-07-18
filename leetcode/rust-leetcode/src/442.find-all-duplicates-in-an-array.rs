use std::collections::HashSet;
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut twice = Vec::new();
        let mut set = HashSet::new();
        for n in nums {
            if set.contains(&n) {twice.push(n);}
            else {set.insert(n);}
        }
        twice
    }
}

/* */

// LEARN
// Even big arrays are faster than hashsets

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut cnt = [0; 100000];
        for i in nums.iter() {
            cnt[*i as usize] += 1;
        }   
        let mut ans: Vec<i32> = vec![];
        for i in 0..cnt.len() {
            if(cnt[i] == 2) {
                ans.push(i as i32);
            }
        }
        ans
    }
}


use std::collections::HashMap;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut map:HashMap<i32, i32> = HashMap::new();
        for n in nums.iter() {
            map.entry(*n).or_insert(1);
        }
        let mut ans:Vec<i32> = Vec::new();
        for i in 1..nums.len()+1 {
            let count = map.entry((i as i32)).or_insert(0);
            if *count == 0 {
                ans.push((i as i32));
            }
        }
        ans
    }
}

impl Solution2 {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = (0..=nums.len() as i32).collect::<Vec<_>>();
        for n in nums {
            ret[n as usize] = 0;
        }
        ret.into_iter().filter(|&n| n != 0).collect()
    }
}

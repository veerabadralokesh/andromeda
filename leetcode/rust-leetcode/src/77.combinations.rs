use std::collections::HashSet;
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let n = n as usize;
        let mut v = vec![0;k];
        let mut nums = [false; 21];
        for i in 1..(n+1) {nums[i]=true;}
        let mut combinations = HashSet::new();
        fn backtrack(n: usize, mut c: &mut HashSet<Vec<i32>>, k: usize, mut nums: &mut[bool], mut v: &mut Vec<i32>, curri: usize) {
            if k == 1 {
                let mut combo = v.clone();
                combo.sort();
                c.insert(combo);
                return;
            }
            for i in (curri+1)..(n+1) {
                if nums[i] {
                    v[k-2] = i as i32;
                    nums[i] = false;
                    backtrack(n, c, k-1, nums, v, i);
                    nums[i] = true;
                }
            }
        }
        backtrack(n, &mut combinations, k+1, &mut nums, &mut v, 0);
        combinations.into_iter().collect()
    }
}

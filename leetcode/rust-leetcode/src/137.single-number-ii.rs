use std::collections::HashSet;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let mut s = 0;
        for n in nums {
            s += n as i64;
            if !set.contains(&n) {
                s -= 3 * (n as i64);
                set.insert(n);
            }
        }
        ((-1*s)/2) as i32
    }
}

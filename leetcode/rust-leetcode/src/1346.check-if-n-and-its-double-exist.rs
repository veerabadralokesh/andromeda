use std::collections::HashSet;
impl Solution {
    pub fn check_if_exist(mut arr: Vec<i32>) -> bool {
        arr.sort_by_key(|&n| -n);
        let mut set = HashSet::new();
        let mut zero_count = 0;
        for &n in arr.iter() {
            if n == 0 {
                zero_count += 1;
            }
            set.insert(n);
        }
        for &n in arr.iter() {
            if set.contains(&(2*n)) {
                if n == 0 && zero_count == 1 {
                    continue;
                }
                return true;
            }
        }
        false
    }
}


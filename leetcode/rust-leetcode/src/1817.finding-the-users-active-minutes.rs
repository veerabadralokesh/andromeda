use std::collections::{HashMap,HashSet};
impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut uam:HashMap<i32, HashSet<i32>> = HashMap::new();
        for log in logs {
            uam.entry(log[0]).or_insert(HashSet::new()).insert(log[1]);
        }
        let mut uuam = vec![0; (k as usize)];
        for am in uam.values() {
            uuam[(am.len()-1) as usize] += 1;
        }
        uuam
    }
}

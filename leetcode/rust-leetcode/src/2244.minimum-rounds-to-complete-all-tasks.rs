impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut counts = HashMap::new();
        for t in tasks {
            *counts.entry(t).or_insert(0) += 1;
        }
        let mut rounds = 0;
        for (t, c) in counts.into_iter() {
            if c == 1 {
                return -1;
            }
            rounds += (c + 2)/3;
        }
        rounds
    }
}


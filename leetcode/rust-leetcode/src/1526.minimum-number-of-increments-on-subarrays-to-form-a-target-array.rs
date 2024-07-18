impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut increments = target.to_vec();
        for i in 1..target.len() {
            increments[i] = (target[i]-target[i-1]).max(0);
        }
        increments.iter().sum()
    }
}

/* */

// LEARN

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        target
            .iter()
            .zip(target.iter().skip(1))
            .fold(target[0], |acc, z| acc + std::cmp::max(0, (z.1 - z.0)))
    }
}

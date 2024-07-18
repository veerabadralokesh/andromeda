impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.into_iter().map(|l| match l.as_str() {
            "../" => -1,
            "./" => 0,
            _ => 1,
        }).fold(0, |acc, x| {
            if x > 0 {
                acc + x
            } else if acc > 0 && x < 0 {
                acc + x
            } else {
                acc
            }
        })
    }
}


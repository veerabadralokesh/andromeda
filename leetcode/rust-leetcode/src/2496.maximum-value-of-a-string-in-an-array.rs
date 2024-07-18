impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.into_iter().map(|s| {
            match s.parse::<i32>() {
                Ok(x) => x,
                Err(x) => s.len() as i32
            }
        }).max().unwrap()
    }
}


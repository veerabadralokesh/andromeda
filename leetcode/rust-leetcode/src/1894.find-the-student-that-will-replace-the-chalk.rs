impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut chalk = chalk.iter().map(|c| *c as i64).collect::<Vec<_>>();
        let mut k = k as i64;
        for i in 1..chalk.len() {
            chalk[i] += chalk[i-1];
        }
        k %= *chalk.last().unwrap();
        match chalk.binary_search(&k) {
            Ok(x) => {x as i32 + 1},
            Err(x) => {x as i32}
        }
    }
}


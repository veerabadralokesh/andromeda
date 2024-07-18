impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let sb = (")".to_owned() + &s).into_bytes();//.map(|b| if b==b'(' {1} else {-1}).collect::<Vec<i8>>();
        let mut dp = vec![0usize; sb.len()];
        for i in 1..dp.len() {
            if sb[i] == b')' && sb[i-dp[i-1]-1] == b'(' {
                dp[i] = dp[i-1] + dp[i- dp[i-1] - 2] + 2;
            }
        }
        *dp.iter().max().unwrap() as i32
    }
}

/* */


impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut best = 0;
        let mut valid_entry = Vec::new();
        let mut count = 0;
        for (mut i, c) in s.chars().enumerate() {
            match c {
                '(' => {
                    count += 1;
                    if count > valid_entry.len() {
                        valid_entry.push(i);
                    }
                }
                ')' => {
                    if count > 0 {
                        count -= 1;
                    } else {
                        valid_entry.clear()
                    }
                    if let Some(v) = valid_entry.get(count) {
                        best = best.max(i - v + 1)
                    }
                    if count + 1 < valid_entry.len() {
                        valid_entry.pop();
                    }
                }
                _ => unreachable!(),
            }
        }
        best as i32
    }
}

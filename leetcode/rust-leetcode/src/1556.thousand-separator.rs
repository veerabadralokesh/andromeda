impl Solution {
    pub fn thousand_separator(mut n: i32) -> String {
        let mut ans = Vec::new();
        let mut i = 0;
        loop {
            ans.push(((n % 10) as u8 + b'0') as char);
            n /= 10;
            i += 1;
            if n > 0 {
                if i % 3 == 0 {
                    ans.push('.');
                }
            } else {
                break;
            }
        }
        ans.reverse();
        ans.iter().collect()
    }
}


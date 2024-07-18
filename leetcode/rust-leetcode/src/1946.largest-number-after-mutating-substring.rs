impl Solution {
    pub fn maximum_number(num: String, change: Vec<i32>) -> String {
        let mut changed = false;
        let mut end = false;
        num.into_bytes().iter().enumerate().map(|(i, &b)| {
            let n = (b - b'0') as i32;
            if n <= change[n as usize] && !end {
                if n < change[n as usize] {
                    changed = true;
                }
                (change[n as usize] as u8 + b'0') as char
            } else {
                if changed {
                    end = true;
                }
                b as char
            }
        }).collect()
    }
}


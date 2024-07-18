impl Solution {
    pub fn digit_sum(mut s: String, k: i32) -> String {
        let k = k as usize;
        let mut new_s: String;
        let mut sb: Vec<u8>;
        let mut count = 0;
        let mut sum = 0;
        while s.len() > k {
            new_s = String::with_capacity(s.len());
            sb = s.into_bytes();
            for (i, &b) in sb.iter().enumerate() {
                sum += (b - b'0') as i32;
                count += 1;
                if count == k || i == sb.len()-1 {
                    new_s.push_str(&sum.to_string());
                    count = 0;
                    sum = 0;
                }
            }
            s = new_s;
        }
        s
    }
}


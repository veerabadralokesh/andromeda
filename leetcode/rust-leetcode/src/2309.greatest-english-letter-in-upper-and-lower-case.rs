impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let (mut chars, mut max_idx, mut idx, mut bit) = ([0; 26], 0, 0, 0);
        let mut ans = String::from("");
        for b in s.into_bytes() {
            if b > b'Z' {
                idx = (b - b'a') as usize;
                bit = 0;
            } else {
                idx = (b - b'A') as usize;
                bit = 1;
            }
            chars[idx] |= (1 << bit);
            if chars[idx] == 3 && idx + 1 > max_idx {
                max_idx = idx + 1;
                ans = format!("{}", if bit == 1 {b as char} else {(b - 32) as char});
            }
        }
        ans
    }
}


impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut counts = [0; 26];
        for b in s.into_bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        let mut changed = true;
        let mut ans = String::new();
        while changed {
            changed = false;
            for i in 0..26 {
                if counts[i] > 0 {
                    counts[i] -= 1;
                    ans.push((i as u8 + b'a') as char);
                    if counts[i] > 0 {
                        changed = true;
                    }
                }
            }
            for i in (0..26).rev() {
                if counts[i] > 0 {
                    counts[i] -= 1;
                    ans.push((i as u8 + b'a') as char);
                    if counts[i] > 0 {
                        changed = true;
                    }
                }
            }
        }
        ans
    }
}


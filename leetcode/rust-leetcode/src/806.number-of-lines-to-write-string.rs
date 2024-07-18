impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut pixels = 0;
        let mut lines = 1;
        let mut word = 0;
        for b in s.into_bytes() {
            word = widths[(b-b'a') as usize];
            if word + pixels <= 100 {
                pixels += word;
            } else {
                lines += 1;
                pixels = word;
            }
        }
        vec![lines, pixels]
    }
}


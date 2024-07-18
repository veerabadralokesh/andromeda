impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut pos = b'a';
        let mut ans = 0;
        let mut dist = 0;
        for b in word.into_bytes() {
            dist = if b > pos {
                (b - pos).min(26 + pos - b)
            } else if b == pos {
                0
            } else {
                (pos - b).min(26 + b - pos)
            };
            pos = b;
            ans += (1 + dist) as i32;
        }
        ans
    }
}


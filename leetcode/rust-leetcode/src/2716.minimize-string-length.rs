impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        let mut counts = [0; 26];
        for b in s.as_bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        counts.iter().filter(|&c| *c > 0).count() as _
    }
}


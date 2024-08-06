impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut counts = [0_i32; 26];
        for b in word.into_bytes() {
            counts[(b-b'a') as usize] += 1;
        }
        counts.sort_by_key(|&c| -c);
        let mut ans = 0;
        for i in 0..26 {
            ans += counts[i as usize] * (1 + (i / 8));
        }
        ans
    }
}


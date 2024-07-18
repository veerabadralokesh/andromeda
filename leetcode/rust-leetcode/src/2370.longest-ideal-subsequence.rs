impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut longest = [0; 26];
        let k = k as usize;
        let s = s.into_bytes().iter().map(|b| (b-b'a') as usize).collect::<Vec<_>>();
        for b in s.into_iter() {
            let mut mr = 0;
            for i in (b.saturating_sub(k)..26.min(b+k+1)) {
                mr = mr.max(longest[i]);
            }
            longest[b] = 1 + mr;
        }
        *longest.iter().max().unwrap()
    }
}
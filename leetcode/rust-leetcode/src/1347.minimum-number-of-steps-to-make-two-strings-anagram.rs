impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut counts:Vec<i32> = vec![0; 26];
        for b in s.bytes() {
            counts[(b-b'a') as usize] += 1;
        }
        for b in t.bytes() {
            counts[(b-b'a') as usize] -= 1;
        }
        counts.iter().map(|c| i32::abs(*c)).sum::<i32>()/2
    }
}

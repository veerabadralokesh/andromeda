impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut c = [0_i32; 26];
        for b in s.into_bytes() {
            c[(b-b'a') as usize] += 1;
        }
        for b in t.into_bytes() {
            c[(b-b'a') as usize] -= 1;
        }
        c.into_iter().map(|x| x.abs()).sum::<i32>()
    }
}


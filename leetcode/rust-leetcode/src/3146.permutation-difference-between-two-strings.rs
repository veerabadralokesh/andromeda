impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut pos = [0; 26];
        for (i,b) in s.into_bytes().iter().enumerate() {
            pos[(b - b'a') as usize] = i as i32;
        }
        let mut ans = 0;
        for (i,b) in t.into_bytes().iter().enumerate() {
            ans += (pos[(b - b'a') as usize] - i as i32).abs();
        }
        ans
    }
}


impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut counts = [0; 26];
        for b in s.into_bytes() {counts[(b-b'a') as usize] += 1;}
        let mut ans = i32::MAX;
        let mut t_counts = [0; 26];
        let tb = target.into_bytes();
        for &b in tb.iter() {
            t_counts[(b-b'a') as usize] += 1;
        }
        for &b in tb.iter() {
            ans = ans.min(counts[(b-b'a') as usize]/t_counts[(b-b'a') as usize]);
        }
        if ans == i32::MAX {0} else {ans}
    }
}


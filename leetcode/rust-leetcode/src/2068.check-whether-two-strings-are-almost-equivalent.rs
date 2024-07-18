impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut counts1 = [0_i32; 26];
        let mut counts2 = [0_i32; 26];
        for b in word1.into_bytes() {
            counts1[(b-b'a') as usize] += 1;
        }
        for b in word2.into_bytes() {
            counts2[(b-b'a') as usize] += 1;
        }
        counts1.into_iter().zip(counts2).all(|(c1,c2)| (c1-c2).abs()<=3)
    }
}


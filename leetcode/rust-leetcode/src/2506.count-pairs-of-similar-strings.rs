impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut wb = words.iter().map(|w| {
            let mut mask = 0;
            for b in w.clone().into_bytes() {
                mask |= 1 << (b - b'a');
            }
            mask
        }).collect::<Vec<_>>();
        let mut ans = 0;
        for i in 0..wb.len()-1 {
            for j in i+1..wb.len() {
                if wb[i] == wb[j] {
                    ans += 1;
                }
            }
        }
        ans
    }
}


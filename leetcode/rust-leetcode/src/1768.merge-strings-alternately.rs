impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let w1 = word1.into_bytes();
        let w2 = word2.into_bytes();
        let mut ans:String = String::new();
        let l1 = w1.len();
        let l2 = w2.len();
        for i in 0..l1.max(l2) {
            if i < l1 {
                ans.push(w1[i] as char);
            }
            if i < l2 {
                ans.push(w2[i] as char);
            }
        }
        ans
    }
}

impl Solution2 {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();
        let mut ans = String::with_capacity(word1.len() + word2.len());
        for _ in 0..word1.len().min(word2.len()) {
            ans.push(chars1.next().unwrap());
            ans.push(chars2.next().unwrap());
        }
        ans.extend(chars1);
        ans.extend(chars2);
        ans
    }
}


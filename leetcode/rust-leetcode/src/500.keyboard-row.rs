impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut chars = [0; 26];
        let rows = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];
        for (i, row) in rows[1..].iter().enumerate() {
            for b in row.to_string().into_bytes() {
                chars[(b - b'a') as usize] = i+1;
            }
        }
        let mut ans = Vec::new();
        for word in words.iter() {
            let wb = word.to_lowercase().into_bytes();
            let base = chars[(wb[0]-b'a') as usize];
            if wb.into_iter().all(|b| {
                chars[(b - b'a') as usize] == base
            }) {
                ans.push(word.clone());
            }
        }
        ans
    }
}


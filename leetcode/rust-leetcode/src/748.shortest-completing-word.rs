impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut counts = [0; 26];
        let mut count = 0;
        for &b in license_plate.to_lowercase().as_bytes() {
            if b < b'a' || b > b'z' {
                continue;
            }
            counts[(b - b'a') as usize] += 1;
            count += 1;
        }
        let contains = |w: &String| -> bool {
            if count > w.len() {
                return false;
            }
            let mut c = [0; 26];
            for &b in w.to_lowercase().as_bytes() {
                if b < b'a' || b > b'z' {
                    continue;
                }
                c[(b - b'a') as usize] += 1;
            }
            (0..26).all(|i| c[i] >= counts[i])
        };
        let (mut ans, mut len) = (String::new(), usize::MAX);
        for w in words.iter() {
            if w.len() < len && contains(&w) {
                ans = w.clone();
                len = w.len();
            }
        }
        ans
    }
}


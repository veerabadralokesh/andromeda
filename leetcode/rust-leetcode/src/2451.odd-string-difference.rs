impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut map = std::collections::HashMap::with_capacity(4);
        let diff_array = |w: String| -> Vec<i8> {
            let wv = w.into_bytes().iter().map(|&b| (b - b'a') as i8).collect::<Vec<_>>();
            (1..wv.len()).map(|i| wv[i]-wv[i-1]).collect()
        };
        for word in words {
            map.entry(diff_array(word.clone())).or_insert(Vec::new()).push(word.clone());
        }
        for (k, v) in map.into_iter() {
            if v.len() == 1 {
                return v[0].clone();
            }
        }
        String::new()
    }
}


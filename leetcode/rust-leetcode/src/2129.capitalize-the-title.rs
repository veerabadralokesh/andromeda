impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let mut ans = Vec::new();
        for word in title.split_whitespace() {
            let mut v: Vec<char> = word.to_lowercase().chars().collect();
            if v.len() > 2 {
                v[0] = v[0].to_uppercase().nth(0).unwrap();
            }
            ans.push(v.into_iter().collect::<String>());
        }
        ans.join(" ")
    }
}


impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let part = part.as_str();
        let mut s = s;
        loop {
            let idx = s.find(part).unwrap_or(s.len());
            if idx == s.len() {
                break s;
            }
            s.replace_range((idx..(idx+part.len())), "");
        }
    }
}

/* */

impl Solution2 {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut result = s;
        while result.contains(&part) {
            result = result.replacen(&part, "", 1)
        }
        result
    }
}

/* */

impl Solution3 {
    pub fn remove_occurrences(mut s: String, part: String) -> String {
        let n=part.len();
        while let Some(i) = s.find(&part) {
            s.replace_range(i..(i+n),"");
        }
        s
    }
}

impl Solution {
    pub fn final_string(s: String) -> String {
        let mut sb:Vec<&str> = s.split("i").collect::<Vec<&str>>();
        let mut ans:String = String::new();
        for ss in sb.iter() {
            ans = ans.chars().rev().collect();
            for c in ss.chars() {
                ans.push(c);
            }
        }
        ans
    }
}

impl Solution2 {
    pub fn final_string(s: String) -> String {
        let mut answer = vec![];
        for byte in s.as_bytes() {
            if byte == &105 {
                answer = answer.into_iter().rev().collect::<Vec<_>>();
                continue;
            }
            answer.push(*byte as char);
        }
        answer.into_iter().collect::<String>()
    }
}
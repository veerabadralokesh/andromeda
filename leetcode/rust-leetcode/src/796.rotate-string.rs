impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            false
        } else {
            let mut s2 = s.clone();
            s2.push_str(s.as_str());
            s2.contains(goal.as_str())
        }
    }
}


use std::collections::HashSet;
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        if s == goal && s.len() > s.chars().collect::<HashSet<char>>().len() {
            return true;
        }
        let sb = s.into_bytes();
        let gb = goal.into_bytes();
        let diff_indices = sb.iter().zip(&gb).enumerate().filter(|&(i, (&a, &b))| a != b).map(|x| x.0).collect::<Vec<_>>();
        diff_indices.len() == 2 && sb[diff_indices[0]] == gb[diff_indices[1]] && sb[diff_indices[1]] == gb[diff_indices[0]]
    }
}


impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {return false;}
        if s1 == s2 {
            return true;
        }
        let sb = s1.into_bytes();
        let gb = s2.into_bytes();
        let diff_indices = sb.iter().zip(&gb).enumerate().filter(|&(i, (&a, &b))| a != b).map(|x| x.0).collect::<Vec<_>>();
        diff_indices.len() == 2 && sb[diff_indices[0]] == gb[diff_indices[1]] && sb[diff_indices[1]] == gb[diff_indices[0]]
    }
}


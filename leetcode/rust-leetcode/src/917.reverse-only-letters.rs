impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut sc = s.chars().collect::<Vec<_>>();
        let (mut l, mut r) = (0, sc.len()-1);
        let is_english = |c| -> bool {
            let b = c as u8;
            (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z')
        };
        while l < r {
            if !is_english(sc[l]) {
                l += 1;
            } else if !is_english(sc[r]) {
                r -= 1;
            } else {
                sc.swap(l, r);
                l += 1;
                r -= 1;
            }
        }
        sc.iter().collect()
    }
}



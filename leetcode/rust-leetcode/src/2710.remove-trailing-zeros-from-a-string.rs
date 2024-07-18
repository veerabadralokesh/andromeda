impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let sc = num.chars().collect::<Vec<_>>();
        let mut i = sc.len()-1;
        while sc[i] == '0' {
            i -= 1;
        }
        sc[..=i].iter().collect()
    }
}


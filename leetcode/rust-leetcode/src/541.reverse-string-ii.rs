impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut sc = s.chars().collect::<Vec<_>>();
        let k = k as usize;
        let l = sc.len();
        for i in (0..l).step_by(2*k) {
            sc[i..(i+k).min(l)].reverse();
        }
        sc.iter().collect()
    }
}


impl Solution {
    pub fn orderly_queue(mut s: String, k: i32) -> String {
        let mut sc = s.clone().chars().collect::<Vec<_>>();
        if k > 1 {
            sc.sort();
            sc.iter().collect()
        } else {
            let mut min_start = sc[0];
            let mut ans = s.clone();
            for i in 1..s.len() {
                if sc[i] <= min_start {
                    min_start = sc[0];
                    let mut ns = sc[i..].iter().cloned().collect::<String>();
                    ns.extend(&sc[..i]);
                    ans = ans.min(ns.to_string());
                }
            }
            ans
        }
    }
}


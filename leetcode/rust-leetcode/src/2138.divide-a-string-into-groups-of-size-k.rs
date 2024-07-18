impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut sc = s.chars().collect::<Vec<_>>();
        let mut ans:Vec<String> = Vec::new();
        let k = k as usize;
        let d = sc.len() / k;
        if k * d < sc.len() {
            for _ in sc.len()..(k*(d+1)) {
                sc.push(fill);
            }
        }
        for i in (0..sc.len()).step_by(k) {
            ans.push(sc[i..i+k].iter().collect());
        }
        ans
    }
}


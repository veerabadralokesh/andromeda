impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let sb = s.into_bytes();
        let mut ans = Vec::with_capacity((1 << sb.len()));
        ans.push(sb.to_vec());
        let mut sv = sb.to_vec();
        let mut rb = b'a';
        for i in 0..sb.len() {
            if sb[i] < b'A' {
                continue;
            }
            rb = if sb[i] < b'a' {
                sb[i] + 32
            } else {
                sb[i] - 32
            };
            for j in 0..ans.len() {
                sv = ans[j].to_vec();
                sv[i] = rb;
                ans.push(sv);
            }
        }
        ans.iter().map(|sv| sv.iter().map(|&b| b as char).collect::<String>()).collect()
    }
}


impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let sc = s.chars()
        .filter(|&c| c != '-')
        .map(|c| if c > 'Z' {(c as u8 - b'a' + b'A') as char} else {c}).collect::<Vec<_>>();
        let k = k as usize;
        let mut ans = String::with_capacity(sc.len() + (sc.len()/k) + 1);
        let (mut i, mut j) = (0, 0);
        for _ in 0..(sc.len() % k) {
            ans.push(sc[i]);
            i += 1;
        }
        if i > 0 && i != sc.len() {ans.push('-')}
        while i < sc.len() {
            ans.push(sc[i]);
            i += 1;
            if j == k-1 && i != sc.len() {
                j = 0;
                ans.push('-');
            } else {
                j += 1;
            }
        }
        ans
    }
}


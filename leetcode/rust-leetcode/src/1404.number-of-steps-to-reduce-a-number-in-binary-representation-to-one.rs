impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut carry = 0;
        let mut ans = 0;
        let sb = s.into_bytes().iter().map(|&b| (b - b'0') as u8).collect::<Vec<_>>();
        for i in (0..sb.len()).rev() {
            if i == 0 && sb[i] == 1 && carry == 0 {
                break;
            }
            let cur = sb[i] + carry;
            match cur {
                2 => {
                    ans += 1;
                    carry = 1;
                },
                1 => {
                    ans += 2;
                    carry = 1;
                },
                _ => {
                    ans += 1;
                    carry = 0;
                }
            }
        }
        ans
    }
}


impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let (mut big, mut big2) = (-1, -1);
        for b in s.into_bytes() {
            if b < b'a' {
                let n = (b - b'0') as i32;
                if n > big {
                    big2 = big;
                    big = n;
                } else if n > big2 && n != big {
                    big2 = n;
                }
            }
        }
        big2
    }
}


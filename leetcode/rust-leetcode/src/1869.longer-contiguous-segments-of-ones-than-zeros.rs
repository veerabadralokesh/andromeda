impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let (mut max1, mut max0, mut c1, mut c0) = (0, 0, 0, 0);
        for i in 0..s.len() {
            match &s[i..i+1] {
                "1" => {
                    c0 = 0;
                    c1 += 1;
                    max1 = c1.max(max1);
                },
                _ => {
                    c1 = 0;
                    c0 += 1;
                    max0 = c0.max(max0);
                }
            }
        }
        max1 > max0
    }
}


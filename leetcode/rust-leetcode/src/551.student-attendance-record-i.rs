impl Solution {
    pub fn check_record(s: String) -> bool {
        let (mut a, mut l) = (0, 0);
        for b in s.into_bytes() {
            match b {
                b'A' => {
                    a += 1;
                    l = 0;
                },
                b'L' => {
                    l += 1;
                    if l == 3 {
                        return false;
                    }
                },
                _ => {
                    l = 0;
                }
            }
        }
        a < 2
    }
}


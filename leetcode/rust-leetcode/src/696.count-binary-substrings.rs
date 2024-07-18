impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let (mut zeros, mut ones, mut prevz, mut prevo, mut ans) = (0, 0, 0, 0, 0);
        for b in s.into_bytes() {
            if b == b'0' {
                if ones > 0 {
                    prevo = ones;
                    ones = 0;
                }
                zeros += 1;
                prevz = 0;
                if prevo > 0 && prevo >= zeros {
                    ans += 1;
                }
            } else {
                if zeros > 0 {
                    prevz = zeros;
                    zeros = 0;
                }
                ones += 1;
                prevo = 0;
                if prevz > 0 && prevz >= ones {
                    ans += 1;
                }
            }
            // println!("{ans} {prevo} {prevz}");
        }
        ans
    }
}


impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut ans = 0;
        for c in 2..=n {
            let c2 = c * c;
            for a in 1..c {
                let a2 = a * a;
                if a2 < c2 {
                    for b in 1..c {
                        if a == b {
                            continue;
                        }
                        if a2 + b * b == c2 {
                            ans += 1;
                        }
                    }
                }
            }
        }
        ans
    }
}


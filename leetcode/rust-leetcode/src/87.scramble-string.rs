use std::collections::HashMap;
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        fn scramble<'a>(s1: &'a str, s2: &'a str, memo: &mut HashMap<(&'a str, &'a str), bool>) -> bool {
            if s1 == s2 {
                return true;
            }
            match memo.get(&(s1, s2)) {
                Some(&b) => b,
                None => {
                    for i in 1..s1.len() {
                        if scramble(&s1[0..i], &s2[0..i], memo) && scramble(&s1[i..], &s2[i..], memo) {
                            memo.insert((s1, s2), true);
                            return true;
                        }
                        if scramble(&s1[0..i], &s2[s2.len()-i..], memo) && scramble(&s1[i..], &s2[..s2.len()-i], memo) {
                            memo.insert((s1, s2), true);
                            return true;
                        }
                    }
                    memo.insert((s1, s2), false);
                    false
                }
            }
        }
        let mut memo = HashMap::new();
        scramble(s1.as_str(), s2.as_str(), &mut memo)
    }
}

/* */

// LEARN

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let mut dp = [[[Option::<bool>::None; 31]; 31]; 31];
        Self::solve(&mut dp, s1.as_bytes(), s2.as_bytes(), 0, s1.len() - 1, 0)
    }

    fn solve(
        dp: &mut [[[Option<bool>; 31]; 31]; 31],
        s1: &[u8],
        s2: &[u8],
        i: usize,
        j: usize,
        x: usize,
    ) -> bool {
        if i >= s1.len() || x >= s1.len() {
            return false;
        }
        if i == j {
            return s1[i] == s2[x];
        }
        if let Some(e) = dp[i][j][x] {
            return e;
        }
        let mut val = false;
        let len = j + 1 - i;
        for k in 1..len {
            if Self::solve(dp, s1, s2, i, i + k - 1, x) && Self::solve(dp, s1, s2, i + k, j, x + k)
                || Self::solve(dp, s1, s2, i + k, j, x)
                    && Self::solve(dp, s1, s2, i, i + k - 1, x + len - k)
            {
                val = true;
                break;
            }
        }
        dp[i][j][x] = Some(val);
        val
    }
}


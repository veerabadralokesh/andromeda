use std::collections::HashMap;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.len() == 0 {
            return s.len() == 0;
        }
        let mut map:HashMap<(usize, usize),bool> = HashMap::new();
        let mut p = p;
        while p.contains("**") {
            p = p.replace("**", "*");
        }
        let s:Vec<u8> = s.into_bytes();
        let p:Vec<u8> = p.into_bytes();
        fn dp(s: &Vec<u8>, p: &Vec<u8>, map: &mut HashMap<(usize, usize), bool>, i: usize, j: usize) -> bool {
            if !map.contains_key(&(i, j)) {
                // println!("{i}, {j}");
                let mut matches:bool = false;
                if j == p.len() {
                    matches = (i == s.len());
                } else if i > s.len() {
                    matches == (j == p.len());
                } else {
                    let first_char_match = i < s.len() && ((p[j] == s[i]) || (p[j] == b'?') || (p[j] == b'*'));
                    if p[j] == b'*' {
                        if j+1 == p.len() {
                            matches = true;
                        } else {
                        matches = dp(s, p, map, i, j+1) || dp(s, p, map, i+1, j);
                        }
                    } else {
                        matches = first_char_match && dp(s, p, map, i+1, j+1);
                    }
                }
                map.insert((i, j), matches);
            }
            *map.get(&(i, j)).unwrap()
        }
        dp(&s, &p, &mut map, 0, 0)
    }
}

/* */

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.len() == 0 {
            return s.len() == 0;
        }
        let sl = s.len();
        let pl = p.len();

        let mut dp = vec![vec![false; pl+1]; sl+1];
        let sb = s.into_bytes();
        let pb = p.into_bytes();

        dp[0][0] = true;

        let is_match = |i: usize, j: usize| sb[i] == pb[j] || pb[j] == b'?';

        for j in 0..pl {
            if pb[j] == b'*' && dp[0][j] {
                dp[0][j+1] = dp[0][j];
            } else {
                break;
            }
        }

        for i in 0..sl {
            for j in 0..pl {
                if pb[j] == b'*' {
                    dp[i+1][j+1] = dp[i+1][j] || dp[i][j+1] // Empty match or Some Match
                } else if is_match(i, j) {
                    dp[i+1][j+1] = dp[i][j];
                }
            }
        }

        dp[sl][pl]
    }
}

use std::collections::HashMap;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.len() == 0 {
            return s.len() == 0;
        }
        let mut map:HashMap<(usize, usize),bool> = HashMap::new();
        let s:Vec<u8> = s.into_bytes();
        let p:Vec<u8> = p.into_bytes();
        fn dp(s: &Vec<u8>, p: &Vec<u8>, map: &mut HashMap<(usize, usize), bool>, i: usize, j: usize) -> bool {
            if !map.contains_key(&(i, j)) {
                let mut matches:bool = false;
                if j == p.len() {
                    matches = (i == s.len());
                } else {
                    let first_char_match = i < s.len() && ((p[j] == s[i]) || (p[j] == b'.'));
                    if j+1 < p.len() && p[j+1] == b'*' {
                        matches = dp(s, p, map, i, j+2) || (first_char_match && dp(s, p, map, i+1, j));
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
    pub fn is_match(input: String, pattern: String) -> bool {
        let input_bytes = input.as_bytes();
        let pattern_bytes = pattern.as_bytes();

        let mut dp = vec![vec![false; pattern.len() + 1]; input.len() + 1];
        dp[0][0] = true;

        for i in 0..input.len() + 1 {
            for j in 1..pattern.len() + 1 {
                // Guarantees there is a prev character.
                if pattern_bytes[j - 1] as char == '*' {
                    // Can we ignore the pattern?
                    dp[i][j] = dp[i][j - 2]
                        // Can we extend the pattern?
                        || (i > 0 && dp[i - 1][j] && (input_bytes[i - 1] == pattern_bytes[j - 2] || pattern_bytes[j - 2] as char == '.'));
                    continue;
                }

                dp[i][j] = i > 0
                    // The previous solution must be valid to extend.
                    && dp[i - 1][j - 1]
                    // The solution can be extended if those chars are equal.
                    && (input_bytes[i - 1] == pattern_bytes[j - 1]
                        || pattern_bytes[j - 1] as char == '.');
            }
        }

        dp[input_bytes.len()][pattern_bytes.len()]
    }
}

/* */


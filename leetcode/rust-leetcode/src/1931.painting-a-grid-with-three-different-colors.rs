use std::collections::HashMap;
static k_mod: i64 = 1_000_000_007;
impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        fn get_color(mask: i64, r: i64) -> i64 {
            (mask >> (r << 1)) & 3
        }
        fn set_color(mask: i64, r: i64, color: i64) -> i64 {
            mask | (color << (r << 1))
        }
        
        fn dp(r: i64, c: i64, prev_col_mask: i64, cur_col_mask: i64, m: i64, n: i64, memo: &mut HashMap<(i64, i64, i64, i64),i64>) -> i64 {
            if c == n {
                return 1;
            }
            if memo.contains_key(&(r, c, prev_col_mask, cur_col_mask)) {
                return *(memo.get(&(r, c, prev_col_mask, cur_col_mask)).unwrap());
            }
            if r == m {
                return dp(0, c+1, cur_col_mask, 0, m, n, memo);
            }
            let mut ans = 0;

            for color in 1..4 {
                if get_color(prev_col_mask, r) == color {
                    continue;
                }
                if r > 0 && get_color(cur_col_mask, r-1) == color {
                    continue;
                }
                ans += dp(r + 1, c, prev_col_mask, set_color(cur_col_mask, r, color), m, n, memo);
                ans %= k_mod;
            }
            memo.insert((r, c, prev_col_mask, cur_col_mask), ans);
            ans
        }

        let mut memo = HashMap::new();
        dp(0, 0, 0, 0, m as i64, n as i64, &mut memo) as _
    }
}


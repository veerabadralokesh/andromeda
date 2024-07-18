impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        let sc = s.chars().collect::<Vec<_>>();
        let mut dp = vec![0; n+1];
        dp[n] = 1;

        fn is_valid_num(a: char, b: char) -> bool {
            if b != 'n' {
                a == '1' || a == '2' && b < '7'
            } else {
                a != '0'
            }
        }

        if is_valid_num(sc[n-1], 'n') {
            dp[n-1] = 1;
        }
        for i in (0..n-1).rev() {
            if is_valid_num(sc[i], 'n') {
                dp[i] += dp[i+1];
            }
            if is_valid_num(sc[i], sc[i+1]) {
                dp[i] += dp[i+2];
            }
        }
        dp[0]
    }
}


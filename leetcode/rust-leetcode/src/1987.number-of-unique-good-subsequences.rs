static k_mod: usize = 1_000_000_007;
impl Solution {
    pub fn number_of_unique_good_subsequences(binary: String) -> i32 {
        let bi = binary.into_bytes().into_iter().map(|b| (b-b'0') as usize).collect::<Vec<_>>();
        let mut dp = [0, 0];
        let mut zero = false;
        for &b in bi.iter() {
            dp[b] = (dp[0] + dp[1]) % k_mod;
            if b == 1 {
                dp[1] += 1;
            } else {
                zero = true;
            }
        }
        // add 1 for '0' if there is zero in the binary string
        ((dp[0]+dp[1] + if zero {1} else {0}) % k_mod) as _
    }
}


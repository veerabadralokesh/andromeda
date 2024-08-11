const dp_len: usize = 1001;
impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        
        let MODULO = 1_000_000_007;

        let nums = nums.iter().map(|&ni| ni as usize).collect::<Vec<_>>();
        let max_num = *nums.iter().max().unwrap();
        let mut dp = [0; dp_len];
        let mut ndp = [0; dp_len];
        for i in 0..=nums[0] {
            dp[i] = 1;
        }

        for i in 1..nums.len() {
            let x = nums[i-1];
            let y = nums[i];
            for j in 1..dp_len {
                dp[j] += dp[j-1];
                if dp[j] >= MODULO {
                    dp[j] -= MODULO;
                }
            }

            for j in 0..=y {
                let mut p = j as i32;
                p = p.min(x as i32 + j as i32 - y as i32);
                ndp[j] = 0;
                if p >= 0 {
                    ndp[j] = dp[p as usize];
                }
            }
            for j in y+1..dp_len {
                ndp[j] = 0;
            }
            (dp, ndp) = (ndp, dp);
        }
        
        let mut ans = 0;

        for c in dp {
            ans += c;
            if ans >= MODULO {
                ans -= MODULO;
            }
        }

        ans as _
    }
}


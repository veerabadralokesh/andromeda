impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        n % 2 == 0
    }
}


/* */

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        
        let mut dp:Vec<bool> = Vec::new();

        for i in 0..=n {
            dp.push(false);
        }

        dp[0] = false;
        dp[1] = false;

        for i in 2usize..=n as usize {

            eprintln!("Compute dp[{i}]");

            let mut win:bool = false;

            for j in 1..i {
                if i % j == 0 {
                    win = win || !dp[i - j];
                }
            }

            dp[i] = win;
        }

        dp[n as usize]
    }
}

/* */

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        
        let mut dp:Vec<bool> = Vec::new();

        for i in 0..=n {
            dp.push(false);
        }

        dp[0] = false;
        dp[1] = false;

        for i in 2usize..=n as usize {

            let mut win:bool = false;

            for j in 1..i {
                if i % j == 0 {
                    win = win || !dp[i - j];
                }
            }

            dp[i] = win;
        }

        dp[n as usize]
    }
}

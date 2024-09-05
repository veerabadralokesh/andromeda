impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let mut rem = mean * (n + rolls.len() as i32) - rolls.iter().cloned().sum::<i32>();
        if rem > n * 6 || rem < n {
            return vec![];
        }
        let mut ans = vec![rem/n; n as usize];
        rem = rem % n;
        for i in 0..rem as usize {
            ans[i]+=1;
        }
        ans
    }
}


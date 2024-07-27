impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let (mut vals, mut counts, mut damage) = (Vec::new(), Vec::new(), 0);
        let mut power = power;//.iter().map(|&p| p as i64).collect::<Vec<_>>();
        power.sort_unstable();
        for p in power {
            if vals.is_empty() || *vals.last().unwrap() != p {
                vals.push(p);
                counts.push(1);
            } else {
                *counts.last_mut().unwrap() += 1;
            }
        }
        let mut dp = vec![0; vals.len()];
        let mut max_prev = 0;
        for i in 0..vals.len() {
            damage = vals[i] as i64 * counts[i];
            max_prev = 0;
            for j in 1..6 {
                if i >= j && vals[i] - vals[i-j] > 2 {
                    max_prev = max_prev.max(dp[i-j]);
                }
            }
            dp[i] = damage + max_prev;
        }
        // println!("{:?}", vals);
        // println!("{:?}", counts);
        // println!("{:?}", dp);
        *dp.iter().max().unwrap()
    }
}


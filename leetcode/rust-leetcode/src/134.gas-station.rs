impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut diff:Vec<i32> = Vec::with_capacity(gas.len());
        for i in 0..gas.len() {
            diff.push(gas[i] - cost[i]);
        }
        let mut net_gain = diff.iter().sum::<i32>();
        if  net_gain < 0 {
            return -1;
        }
        // println!("{:?}", diff);
        let mut cumulative_diff = 0i32;
        let mut ans:i32 = -1;
        for (i, d) in diff.into_iter().enumerate() {
            cumulative_diff += d;
            if cumulative_diff < 0 {
                cumulative_diff = 0;
                ans = -1;
                continue;
            }
            if ans == -1 {
                ans = i as i32;
            }
        }
        ans
    }
}

/*
*/

impl Solution2 {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let s1: i32 = gas.iter().sum();
        let s2: i32 = cost.iter().sum();

        if s1 < s2 { return -1; }

        let mut tank = 0;
        let mut idx = 0;

        for i in 0..gas.len() {
            tank += (gas[i] - cost[i]);

            if tank < 0 {
                let i = i as i32;
                
                tank = 0;
                idx = i + 1;
            }
        }

        idx
    }
}
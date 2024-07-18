impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 3 {
            return n;
        }
        let n = n as usize;
        let mut ways = vec![0; n + 1];
        ways[1] = 1;
        ways[2] = 2;
        for i in 3..=n {
            ways[i] += ways[i-2] + ways[i-1];
        }
        ways[n]
    }
}


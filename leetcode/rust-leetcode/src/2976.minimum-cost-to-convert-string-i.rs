impl Solution {
    pub fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {
        let mut cc = vec![vec![u64::MAX; 26]; 26];
        let (mut o, mut c) = (0, 0);
        for i in 0..original.len() {
            o = original[i] as usize - 97;
            c = changed[i] as usize - 97;
            cc[o][c] = cc[o][c].min(cost[i] as u64);
        }
        for k in 0..26 {
            for i in 0..26 {
                if cc[i][k] < u64::MAX {
                    for j in 0..26 {
                        if cc[k][j] < u64::MAX {
                            cc[i][j] = cc[i][j].min(cc[i][k] + cc[k][j]);
                        }
                    }
                }
            }
        }
        let mut ans = 0;
        for (s, t) in source.as_bytes().into_iter().zip(target.as_bytes()) {
            if s != t {
                (o, c) = ((s - b'a') as usize, (t-b'a') as usize);
                if cc[o][c] == u64::MAX {
                    return -1;
                }
                ans += cc[o][c];
            }
        }
        ans as _
    }
}


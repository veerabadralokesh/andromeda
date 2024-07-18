impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let (mut start, mut end, n) = (0, 0, n as usize);
        let mut sectors = vec![0; n+1];
        let rounds = rounds.into_iter().map(|r| r as usize).collect::<Vec<_>>();
        sectors[rounds[0]] = 1;
        for i in 1..rounds.len() {
            start = rounds[i-1] as usize;
            end = rounds[i] as usize;
            if end < start {
                for j in start+1..=n {
                    sectors[j] += 1;
                }
                for j in 1..=end {
                    sectors[j] += 1;
                }
            } else {
                for j in start+1..=end {
                    sectors[j] += 1;
                }
            }
        }
        let max_visits = *sectors.iter().max().unwrap();
        (1..=n).filter(|&i| sectors[i]==max_visits).map(|i| i as i32).collect()
    }
}


// Bottom Up
static k_mod: i64 = 1_000_000_007;
impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let n = locations.len();
        let fuel = fuel as usize;
        let (start, finish) = (start as usize, finish as usize);
        let mut dp = vec![vec![0; fuel+1]; n+1];
        for f in 0..=fuel {
            dp[finish][f] = 1;
        }
        let mut fuel_ij = 0;
        for f in 1..=fuel {
            for i in 0..n {
                for j in 0..n {
                    if i != j {
                        fuel_ij = (locations[i] - locations[j]).abs() as usize;
                        if fuel_ij <= f {
                            dp[i][f] = (dp[i][f] + dp[j][f - fuel_ij]) % k_mod;
                        }
                    }
                }
            }
        }
        dp[start][fuel] as _
    }
}

/* */

// Top Down
use std::collections::HashMap;
static k_mod: i64 = 1_000_000_007;
impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let n = locations.len();
        let (start, finish) = (start as usize, finish as usize);
        
        fn dfs(start: usize, fuel: i32, finish: usize, locations: &Vec<i32>, map: &mut HashMap<(usize, i32), i64>) -> i64 {
            if fuel < 0 {
                return 0;
            }
            match map.get(&(start, fuel)) {
                Some(&count) => count,
                None => {
                    let mut count = if finish == start {1} else {0};
                    for (i, &loc) in locations.iter().enumerate() {
                        if i != start {
                            count = (count + dfs(i, fuel - (locations[start]-loc).abs(), finish, locations, map)) % k_mod;
                        }
                    }
                    map.insert((start, fuel), count);
                    count
                }
            }
        }
        
        let mut map = HashMap::new();
        dfs(start, fuel, finish, &locations, &mut map) as _
    }
}


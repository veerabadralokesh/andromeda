use std::collections::{BTreeSet,HashMap};
impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut graph = HashMap::with_capacity(n);
        for road in roads.iter() {
            graph.entry(road[0]).or_insert(Vec::new()).push(road[1]);
            graph.entry(road[1]).or_insert(Vec::new()).push(road[0]);
        }
        let mut set = BTreeSet::new();
        for (city, roads) in graph.into_iter() {
            set.insert((roads.len(), city));
        }
        let mut ans = 0;
        for i in (1..=n).rev() {
            ans += (i * set.pop_last().unwrap().0);
            if set.is_empty() {
                break;
            }
        }
        ans as _
    }
}

/* */

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let n = n as i64;
        let mut counts = vec![0; n as usize];
        for road in roads.iter() {
            counts[road[0] as usize] += 1;
            counts[road[1] as usize] += 1;
        }
        counts.sort_by_key(|&n| -n);
        let mut ans = 0;
        for (i, roads) in (1..=n).rev().enumerate() {
            ans += (roads * counts[i]);
        }
        ans as _
    }
}

/* */

use std::collections::*;
impl Solution {
    pub fn maximum_importance(n: i32, mut roads: Vec<Vec<i32>>) -> i64 {
        let mut graph = roads.iter().fold(vec![0i64; n as usize], |mut acc, road| {
            acc[road[0] as usize] += 1;
            acc[road[1] as usize] += 1;
            acc
        });

        graph.sort_unstable();
        let mut total = 0;
        for i in 1..=n {
            total += graph[i as usize - 1] * i as i64
        }
        total
    }
}



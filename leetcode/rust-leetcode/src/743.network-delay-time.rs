impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize - 1;
        let mut net = vec![vec![i32::MAX; n]; n];
        let (mut u, mut v, mut w) = (0, 0, 0);
        for t in times {
            (u, v, w) = (t[0] as usize - 1, t[1] as usize - 1, t[2]);
            net[u][v] = w;
        }
        for i in 0..n {
            net[i][i] = 0;
        }
        for m in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if i != j && net[i][m] != i32::MAX && net[m][j] != i32::MAX {
                        net[i][j] = net[i][j].min(net[i][m] + net[m][j]);
                    }
                }
            }
        }
        let max_time = *net[k].iter().max().unwrap();
        if max_time == i32::MAX {-1} else {max_time}
    }
}

/* */

// LEARN

use std::collections::BinaryHeap;

const INF: i32 = 1e9 as i32;


impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut dist = vec![INF; n];
        let mut graph = vec![Vec::<(usize, i32)>::new(); n];

        dist[(k - 1) as usize] = 0;
        for edge in times {
            graph[(edge[0] - 1) as usize].push(
                ((edge[1] - 1) as usize, edge[2])
            );
        }

        let mut queue = BinaryHeap::from([(0, (k - 1) as usize)]);

        while let Some((cost, from)) = queue.pop() {
            if cost > dist[from] {
                continue;
            }
            for &(to, weight) in &graph[from] {
                if (dist[from] + weight) < dist[to] {
                    dist[to] = dist[from] + weight;
                    queue.push((dist[to], to));
                }
            }
        }

        let maxi = *dist.iter().max().unwrap();

        if maxi == INF {
            return -1;
        }
        return maxi;
    }
}


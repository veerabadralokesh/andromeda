use std::collections::{HashMap,HashSet,VecDeque};
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let mut route_buses = HashMap::new();
        for (bus, route) in routes.iter().enumerate() {
            for &r in route.iter() {
                route_buses.entry(r).or_insert(HashSet::new()).insert(bus);
            }
        }
        let mut q = VecDeque::new();
        q.push_back(source);
        let mut min_buses = 0;
        let mut visited = HashSet::new();
        let mut used_buses = HashSet::new();
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(r) = q.pop_front() {
                    if visited.contains(&r) {
                        continue;
                    }
                    if r == target {
                        return min_buses;
                    }
                    visited.insert(r);
                    if route_buses.get(&r).is_none() {
                        continue;
                    }
                    for &bus in route_buses.get(&r).unwrap().iter() {
                        if used_buses.contains(&bus) {
                            continue;
                        }
                        used_buses.insert(bus);
                        for &nr in routes[bus].iter() {
                            if nr == target {
                                return min_buses+1;
                            }
                            if !visited.contains(&nr) {
                                q.push_back(nr);
                            }
                        }
                    }
                }
            }
            min_buses += 1;
        }
        -1
    }
}


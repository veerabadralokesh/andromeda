impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let (s, d) = if start > destination {(destination as usize, start as usize)} else {(start as usize, destination as usize)};
        if s == d {
            return 0;
        }
        let n = distance.len();
        let (mut fwd, mut bwd) = (0, 0);
        for i in s..d {
            fwd += distance[i];
        }
        for i in d..s+n {
            bwd += distance[i%n];
        }
        fwd.min(bwd)
    }
}


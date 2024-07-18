impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        if dist.len() > (hour.ceil() as usize) {
            return -1;
        }
        let mut start = 1;
        let mut end = i32::MAX-1;
        let mut min_speed = i32::MAX-1;
        let l = dist.len();
        let dist = dist.into_iter().map(|d| d as f64).collect::<Vec<_>>();
        while start <= end {
            let mid = ((start+end)/2) as f64;
            let mut time_taken = 0.0_f64;
            for i in 0..l-1 {
                time_taken += (dist[i]/ mid).ceil();
                if time_taken > hour {
                    break;
                }
            }
            time_taken += (dist[l-1]/ mid);
            let mid = mid as i32;
            if time_taken <= hour {
                min_speed = min_speed.min(mid);
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        min_speed
    }
}

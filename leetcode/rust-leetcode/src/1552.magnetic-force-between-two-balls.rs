impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort();
        let place_balls_with_dist = |dist: i32| -> i32 {
            let mut ball_pos = i32::MIN;
            let mut balls = 0;
            for &p in position.iter() {
                if p >= ball_pos + dist {
                    ball_pos = p;
                    balls += 1;
                }
            }
            balls
        };
        let (mut low, mut high) = (1, *position.last().unwrap() - position[0]);
        while low < high {
            // let mid = low + (high - low) / 2;
            // let mid = high - (high - low) / 2;
            let mid = low + (high - low) / 2 + 1;
            let x = place_balls_with_dist(mid);
             if x >= m {
                low = mid;
            } else {
                high = mid-1;
            }
        }
        low
    }
}

/* */

// LEARN


impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();
        let first = position[0];
        let max_diff = position.iter().last().unwrap() - first;
        let (mut l, mut r) = (1, max_diff / (m - 1));
        while l < r {
            let mid = (l + r) / 2 + 1;
            if position.iter().fold((1, first), |(acc, prev), &i| if i - prev >= mid {(acc + 1, i)} else {(acc, prev)}).0 >= m {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        l
    }
}



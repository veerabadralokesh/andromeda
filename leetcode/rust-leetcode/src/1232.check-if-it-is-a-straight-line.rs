impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let mut point = coordinates[0].to_vec();

        fn slope(dx: i32, dy: i32) -> Option<f64> {
            if dy == 0 {
                Some(f64::MAX)
            } else {
                Some(dx as f64 / dy as f64)
            }
        }
        let mut s = None;
        for (i, c) in coordinates.iter().skip(1).enumerate() {
            if s.is_none() {
                s = slope(c[0]-point[0], c[1]-point[1]);
            } else {
                if s != slope(c[0]-point[0], c[1]-point[1]) {
                    return false;
                }
            }
        }
        true
    }
}


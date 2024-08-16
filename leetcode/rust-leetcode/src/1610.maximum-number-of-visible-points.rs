use std::f64::consts::PI;
impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        let mut same = 0;
        let (x, y) = (location[0], location[1]);
        let mut max_visible = 0;
        let angle = angle as f64;
        let mut angles = Vec::with_capacity(points.len());
        let get_angle = |px: i32, py: i32| -> f64 {
            f64::atan2((py-y) as f64, (px-x) as f64) * 180.0 / PI
        };
        for p in points {
            if p[0] == x && p[1] == y {
                same += 1;
            } else {
                angles.push(get_angle(p[0], p[1]));
            }
        }
        angles.sort_by(|a1, a2| {
            a1.partial_cmp(&a2).unwrap()
        });
        for i in 0..angles.len() {
            angles.push(angles[i] + 360.0);
        }
        let mut j = 0;
        for i in 0..angles.len() {
            while angles[i] - angles[j] > angle {
                j += 1;
            }
            max_visible = max_visible.max(i - j + 1);
        }
        max_visible as i32 + same
    }
}


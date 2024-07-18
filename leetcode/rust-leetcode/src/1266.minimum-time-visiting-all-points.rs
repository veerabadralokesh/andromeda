impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut t:i32 = 0;
        for i in 1..points.len() {
            let p1 = &points[i-1];
            let p2 = &points[i];
            let dx = (p1[0]-p2[0]).abs();
            let dy = (p1[1]-p2[1]).abs();
            t += dx.min(dy) + (dx - dy).abs();
        }
        t
    }
}

impl Solution2 {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut t = 0;

        for (i, &ref point) in points.iter().skip(1).enumerate() {
            t += (point[0] - points[i][0]).abs().max((point[1] - points[i][1]).abs());
        }

        t         
    }
}

impl Solution3 {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
      let mut res = 0;
      let (mut prev_x, mut prev_y) = (points[0][0], points[0][1]);
  
      for i in 1 .. points.len() {
        let (x, y) = (points[i][0], points[i][1]);
        res += (x - prev_x).abs().max((y - prev_y).abs());
        prev_x = x;
        prev_y = y;
      }
  
      return res;
    }
  }
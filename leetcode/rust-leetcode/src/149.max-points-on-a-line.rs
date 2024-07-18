use std::collections::HashMap;
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n < 3 {
            return n as i32;
        }
        let mut counts = HashMap::new();//:HashMap<Vec<f32,f32>, i32>
        for i in 0..(n-1) {
            for j in (i+1)..n {
                let x1 = points[i][0] as f32;
                let y1 = points[i][1] as f32;
                let x2 = points[j][0] as f32;
                let y2 = points[j][1] as f32;
                
                let mut slope = f32::MAX;
                let mut yc = f32::MAX;
                if x1 != x2 {
                    slope = (y2-y1)/(x2-x1);
                    if slope == 0.0 {
                        slope = 0.0;
                    }
                    yc = y1 - slope * x1;
                    if yc == 0.0 {
                        yc = 0.0;
                    }
                } else {
                    yc = x1;
                }
                let key = format!("{:.4}_{:.4}", slope, yc);
                *counts.entry(key).or_insert(0) += 1;
            }
        }
        let mut max_count = *counts.values().max().unwrap();
        max_count = (1 + f32::sqrt((1 + 8 * max_count) as f32) as i32)/2;
        max_count
    }
}

/*
*/

// LEARN

use std::collections::HashMap;

impl Solution {
    pub fn max_points(mut points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();

        (0..n - 1)
            .map(|i| {
                let (x1, y1) = (points[i][0], points[i][1]);
                *(i + 1..n)
                    .fold(HashMap::with_capacity(n - i - 1), |mut acc, j| {
                        let (x2, y2) = (points[j][0], points[j][1]);
                        let x_diff = (x2 - x1);
                        let y_diff = (y2 - y1);
                        let slope = match (x_diff == 0, y_diff == 0) { 
                            (true, _) => 0.0,
                            (_, true) => f64::INFINITY,
                            _ => y_diff as f64 / x_diff as f64,
                        };
                        *acc.entry(unsafe { std::mem::transmute::<f64, u64>(slope) })
                            .or_default() += 1;
                        acc
                    })
                    .values()
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap_or(0)
            + 1
    }
}

/*
*/

use std::collections::HashMap;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let points: Vec<(i32, i32)> = points.into_iter().map(|x| (x[0], x[1])).collect();
        let mut max_cnt = 0;
        if points.len() == 1 {
            return 1;
        }
        let mut cnt_by_direction: HashMap<u32, i32> = HashMap::new();
        for i in 0..points.len() {
            let a = points[i];
            cnt_by_direction.clear();
            for j in i + 1..points.len() {
                let b = points[j];
                let d = (b.0 - a.0, b.1 - a.1);
                let slope = if d.1 == 0 {
                    0.0
                } else if d.0 == 0 {
                    f32::MAX
                } else {
                    (d.1 as f32) / (d.0 as f32)
                };
                let e = cnt_by_direction.entry(slope.to_bits()).or_insert(0);
                *e += 1;
                if *e > max_cnt {
                    max_cnt = *e;
                }
            }
        }
        max_cnt + 1
    }
}

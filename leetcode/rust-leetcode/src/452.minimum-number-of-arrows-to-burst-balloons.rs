impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 1 {
            return 1i32;
        }
        points.sort();
        let mut intersections = Vec::new();
        intersections.push(points[0].to_vec());
        let mut count = 1usize;
        for point in points.iter().skip(1) {
            if intersections[count-1][1] < point[0] {
                intersections.push(point.to_vec());
                count += 1;
            } else {
                intersections[count-1][1] = intersections[count-1][1].min(point[1]);
            }
        }
        // println!("{:?}", intersections);
        count as i32
    }
}

/* */

// LEARN

use std::cmp::Ordering;

#[derive(Eq)]
struct Interval {
    start: i32,
    end: i32,
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut intervals: Vec<Interval> = points
        .into_iter()
        .map(|v| Interval {
            start: v[0],
            end: v[1],
        })
        .collect();
        intervals.sort_unstable();
        let mut total_arrows = 0;
        let mut opt_min_end_cur_balloons = None;
        for interval in intervals.iter() {
            match opt_min_end_cur_balloons {
                None => {
                    opt_min_end_cur_balloons = Some(interval.end);
                }
                Some(min_end_cur_balloons) => {
                    if min_end_cur_balloons < interval.start {
                        opt_min_end_cur_balloons = Some(interval.end);
                        total_arrows += 1;
                    } else {
                        if interval.end < min_end_cur_balloons {
                            opt_min_end_cur_balloons = Some(interval.end);
                        }
                    }
                }
            }
        }
        total_arrows + 1
    }
}

/* */

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort();
        let mut arrows = Vec::new();
        let mut current_arrow = points[0].to_vec();
        for point in points.iter().skip(1) {
            if point[0] > current_arrow[1] {
                arrows.push(current_arrow);
                current_arrow = point.to_vec();
            } else {
                current_arrow[1] = current_arrow[1].min(point[1]);
            }
        }
        arrows.len() as i32 + 1
    }
}

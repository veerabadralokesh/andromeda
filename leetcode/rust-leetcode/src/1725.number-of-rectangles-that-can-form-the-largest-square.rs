impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut max_side = 0;
        let mut count = 0;
        for rect in rectangles.iter() {
            let mut side = std::cmp::min(rect[0], rect[1]);
            if max_side == side {
                count += 1;
            } else if side > max_side {
                max_side = side;
                count = 1;
            }
        }
        count
    }
}


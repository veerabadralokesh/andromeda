impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let bl = bottom_left.into_iter().map(|v| (v[0] as i64, v[1] as i64)).collect::<Vec<_>>();
        let tr = top_right.into_iter().map(|v| (v[0] as i64, v[1] as i64)).collect::<Vec<_>>();
        let mut ans = 0;
        for i in 0..bl.len()-1 {
            for j in i+1..bl.len() {
                ans = ans.max(Solution::max_squqre(
                    bl[i].0, bl[i].1, tr[i].0, tr[i].1,
                    bl[j].0, bl[j].1, tr[j].0, tr[j].1,
                ));
            }
        }
        ans*ans
    }

    pub fn max_squqre(ax1: i64, ay1: i64, ax2: i64, ay2: i64, bx1: i64, by1: i64, bx2: i64, by2: i64) -> i64 {
        let (cx1, cy1) = (ax1.max(bx1), ay1.max(by1));
        let (cx2, cy2) = (ax2.min(bx2), ay2.min(by2));
        // let mut ans = (ax2-ax1) * (ay2-ay1) + (bx2-bx1) * (by2-by1);
        if cx2 >= cx1 && cy2 >= cy1 {
            (cx2-cx1).min(cy2-cy1)
        } else {
            0
        }
    }
}


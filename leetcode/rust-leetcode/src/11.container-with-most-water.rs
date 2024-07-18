impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut lp:usize = 0;
        let mut rp:usize = height.len() - 1;
        
        let mut max_area:i32 = 0;

        while lp < rp {
            let lv = height[lp];
            let rv = height[rp];
            let area:i32 = lv.min(rv) * (rp - lp) as i32;
            max_area = max_area.max(area);
            if lv < rv {
                lp += 1;
            } else {
                rp -= 1;
            }
        }
        max_area
    }
}
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() == 1 {
            return 0;
        }
        let mut remove = 0i32;
        intervals.sort_by_key(|i| i[1]);
        let mut end = i32::MIN;
        for i in intervals {
            if i[0] >= end {
                end = i[1];
            } else {
                remove += 1;
            }
        }
        remove
    }
}

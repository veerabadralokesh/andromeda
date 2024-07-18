impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        let mut ans = vec![];
        if intervals.is_empty() {
            return ans;
        }
        intervals.sort_by_key(|x| x[0]);
        let mut p = intervals[0].clone();
        for x in intervals {
            if x[0] > p[1] {
                ans.push(p);
                p = x;
            } else {
                p[1] = p[1].max(x[1]);
            }
        }
        ans.push(p);
        ans
    }
}


/*
*/



impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        
        let mut changed = true;
        while changed {
            changed = false;
            intervals.sort_by_key(|i| (i[0], i[1]));
            let mut new_intervals:Vec<Vec<i32>> = Vec::new();
            new_intervals.push(intervals[0].to_vec());
            let mut size:usize = 0;
            for interval in intervals.iter().skip(1) {
                // println!("{:?}, {:?}", new_intervals, interval);
                if interval[0] <= new_intervals[size][1] {
                    if interval[1] > new_intervals[size][1] {
                        new_intervals[size][1] = interval[1];
                        changed = true;
                    }
                } else {
                    new_intervals.push(interval.to_vec());
                    size += 1;
                }
            }
            intervals = new_intervals;
        }

        intervals
    }
}
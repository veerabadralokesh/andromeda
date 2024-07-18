impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity(intervals.len() + 1);

        for (i, interval) in intervals.iter().enumerate() {
            if interval[0] > new_interval[1] {
                ans.push(new_interval);
                for j in i..intervals.len() {
                    ans.push(intervals[j].to_vec());
                }
                return ans;
            } else if interval[1] < new_interval[0] {
                ans.push(interval.to_vec());
            } else {
                new_interval = vec![new_interval[0].min(interval[0]), new_interval[1].max(interval[1])];
            }
        }

        ans.push(new_interval);

        ans
    }
}

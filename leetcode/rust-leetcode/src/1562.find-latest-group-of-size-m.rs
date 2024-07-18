impl Solution {
    pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        let m = m as usize;
        if m == arr.len() {return m as i32;}
        let arr = arr.iter().map(|&n| n as usize).collect::<Vec<_>>();
        let mut ranges = vec![0; arr.len() + 2];
        let mut step = 0;
        let mut ans = -1;
        let (mut end, mut start) = (0, 0);
        for &n in arr.iter() {
            if ranges[n - 1] == m || ranges[n + 1] == m {
                ans = step;
            }
            step += 1;
            end = n + ranges[n + 1];
            start = n - ranges[n - 1];
            ranges[start] = end - start + 1;
            ranges[end] = end - start + 1;
            // println!("{:?}", ranges);
        }
        ans
    }
}


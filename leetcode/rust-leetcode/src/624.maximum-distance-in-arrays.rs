impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let (mut ans, mut mini, mut maxi, mut last) = (0, 1000000, -1000000, 0);
        for arr in arrays.iter() {
            last = arr.len() - 1;
            ans = ans.max(maxi - arr[0]).max(arr[last] - mini);
            maxi = maxi.max(arr[last]);
            mini = mini.min(arr[0]);
        }
        ans
    }
}


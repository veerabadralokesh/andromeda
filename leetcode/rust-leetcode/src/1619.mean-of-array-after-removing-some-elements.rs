impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        arr.sort();
        let mut l = arr.len();
        let start = l / 20;
        let end = start * 19;
        let mut sum = 0f64;
        for i in start..end {
            sum += (arr[i] as f64);
        }
        sum / ((end-start) as f64)
    }
}


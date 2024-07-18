impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        if k == 1 || k == weights.len() {
            return 0;
        }
        let n = weights.len();
        let mut ans = 0i64;
        let weights = weights.iter().map(|&w| w as i64).collect::<Vec<_>>();
        let mut sums = vec![0; n-1];
        for i in 0..n-1 {
            sums[i] = weights[i] + weights[i+1];
        }
        sums.sort();
        for i in 0..k-1 {
            ans += (sums[n-2-i] - sums[i]);
        }
        ans
    }
}

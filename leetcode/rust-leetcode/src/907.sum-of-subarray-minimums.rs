
static modulo: i64 = 1000000007;
impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        // println!("{n}");
        // let mut sums = vec![0; n];
        let mut ans = 0i64;
        // for i in 0..n {
        //     sums[i] = arr[i];
        //     ans += arr[i];
        // }
        // ans %= modulo;
        // for i in 1..n {
        //     let mut nsums = vec![0; n-i];
        //     for j in 0..n-i {
        //         nsums[j] = sums[j].min(sums[j+1]);
        //         ans += nsums[j];
        //     }
        //     ans %= modulo;
        //     sums = nsums;
        // }
        // for sum in sums {
        //     println!("{:?}", sum);
        // }
        let mut stack:Vec<usize> = Vec::with_capacity(n);
        for i in 0..=n {
            while !stack.is_empty() && (i == n || arr[(*stack.last().unwrap())] >= arr[i]) {
                let big_idx = stack.pop().unwrap();
                let (left, right) = match stack.last() {
                    Some(idx) => {
                        ((big_idx - idx) as i64, (i - big_idx) as i64)
                    },
                    _ => {
                        (big_idx as i64 + 1, (i - big_idx) as i64)
                    }
                };
                ans += arr[big_idx] as i64 * left * right;
                ans %= modulo;
            }
            stack.push(i);
        }
        ans as i32
    }
}

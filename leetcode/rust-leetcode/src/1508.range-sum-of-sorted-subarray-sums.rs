impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let modulo = 1_000_000_007;
        let n = n as usize;
        let l = left as usize;
        let r = right as usize;
        let mut sums = Vec::with_capacity((n * (n+1)) / 2);
        let mut sum = 0;
        for i in 0..n {
            sum = 0;
            for &num in nums.iter().skip(i) {
                sum += num;
                sums.push(sum);
            }
        }
        sums.sort_unstable();
        // println!("{:?}", sums);
        let mut ans = 0;
        for i in l-1..r {
            ans = (ans + sums[i]) % modulo;
        }
        ans
    }
}


impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let mut ans = vec![0,0];
        let (mut min_diff, mut x, mut y) = (i32::MAX, 0, 0);
        for n in num+1..num+3 {
            x = (f64::sqrt(n as f64)) as i32;
            while x > 0 {
                if n % x == 0 {
                    y = n / x;
                    if (x - y).abs() < min_diff {
                        ans[0] = x;
                        ans[1] = y;
                        min_diff = (x - y).abs();
                    }
                }
                x -= 1;
            }
        }
        ans
    }
}


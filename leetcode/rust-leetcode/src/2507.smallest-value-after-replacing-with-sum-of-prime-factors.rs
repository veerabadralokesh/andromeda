impl Solution {
    pub fn smallest_value(n: i32) -> i32 {
        let n = n as usize;
        let mut primes = vec![true; n + 1];
        primes[1] = false;
        for i in 2..=(f64::sqrt(n as f64) as usize) {
            if primes[i] {
                for j in ((i << 1)..=n).step_by(i) {
                    primes[j] = false;
                }
            }
        }
        let pnums = (2..=n).filter(|&i| primes[i]).collect::<Vec<_>>();
        let mut ans = n;
        let mut x = n;
        while !primes[ans] {
            x = ans;
            ans = 0;
            for &p in pnums.iter() {
                while p <= x && x % p == 0 {
                    ans += p;
                    x /= p;
                }
            }
            if ans == n {
                break;
            }
        }
        ans as _
    }
}


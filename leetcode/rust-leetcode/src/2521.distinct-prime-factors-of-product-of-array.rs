impl Solution {
    pub fn distinct_prime_factors(nums: Vec<i32>) -> i32 {
        let n = 1000;
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
        // let mut prod = nums.iter().fold(1, |acc, &x| acc * x) as usize;
        let mut count = 0;
        for n in nums.into_iter() {
            let mut prod = n as usize;
            for &p in pnums.iter() {
                while p <= prod && prod % p == 0 {
                    prod /= p;
                    if primes[p] {
                        primes[p] = false;
                        count += 1;
                    }
                }
            }
        }
        count
    }
}


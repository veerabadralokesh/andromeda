impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let (l, r) = (left as usize, right as usize);
        let mut primes = vec![true; r+1];
        primes[0] = false;
        primes[1] = false;
        let sqrtn = f64::sqrt(r as f64) as usize;
        for i in 2..=sqrtn {
            if primes[i] {
                for j in (2*i..=r).step_by(i) {
                    primes[j] = false;
                }
            }
        }
        let mut p = Vec::with_capacity(r - l + 1);
        for i in l..=r {
            if primes[i] {
                p.push(i);
            }
        }
        let (mut mind, mut ans) = (usize::MAX, vec![-1, -1]);
        for i in 1..p.len() {
            if p[i] - p[i-1] < mind {
                mind = p[i] - p[i-1];
                ans[0] = p[i-1] as i32;
                ans[1] = p[i] as i32;
            }
        }
        ans
    }
}


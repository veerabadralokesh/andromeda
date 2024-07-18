impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }
        let n = (n as usize);
        let mut vec = vec![true; n];
        vec[0] = false;
        vec[1] = false;
        let sqrtn = f64::sqrt(n as f64) as usize;
        let mut count = 0;
        for i in 2..=sqrtn {
            if vec[i] {
                for j in (2*i..n).step_by(i) {
                    vec[j] = false;
                }
                count += 1;
            }
        }
        vec.into_iter().filter(|&b| b).count() as _
    }
}


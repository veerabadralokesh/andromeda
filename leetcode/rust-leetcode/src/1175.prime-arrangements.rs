static MODULO: usize = 1_000_000_007;
impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        if n < 3 {
            return 1;
        }
        let n = (n as usize) + 1;
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
        let primes = vec.into_iter().filter(|&b| b).count() as usize;
        let non_primes = n - primes - 1;
        let mut factorials = vec![0; 101]; factorials[0] = 1; factorials[1] = 1;
        fn factorial(x: usize, f: &mut Vec<usize>) -> usize {
            if f[x] == 0 {
                f[x] = (x * factorial(x-1, f)) % MODULO;
            }
            f[x]
        }
        let ans = (factorial(primes, &mut factorials) * factorial(non_primes, &mut factorials)) % MODULO;
        // println!("{primes} {non_primes} {:?}", factorials);
        ans as _
    }
}


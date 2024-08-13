impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut n = n;
        let mut i = 0;
        let mut p = 1;
        while p*3 <= n {
            p *= 3;
            i += 1;
        }
        // println!("{n} {p} {i}");
        while n > 0 && i > -1 {
            if n >= p {
                n -= p;
            }
            p /= 3;
            i -= 1;
            // println!("{n} {p} {i}");
        }
        n == 0
    }
}


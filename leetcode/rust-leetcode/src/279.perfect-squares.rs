use std::cmp::min;
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut sq = Vec::new();
        let mut j = 0;
        for i in 1..=n {
            j = i*i;
            if j == n {return 1;}
            if j < n {
                sq.push(j)
            } else {
                break;
            }
        }
        let mut ps = vec![n; n+1];
        ps[0] = 0;
        ps[1] = 1;
        for i in 2..=n {
            for &j in &sq {
                if j > i {
                    break;
                }
                ps[i] = min(ps[i], ps[i-j]+1);
            }
        }
        ps[n] as i32
    }
}

/* */

impl Solution {
    pub fn num_squares(mut n: i32) -> i32 {
        let sqrtn = (f32::sqrt(n as f32)) as i32;
        if sqrtn * sqrtn == n {
            return 1;
        }
        let ncopy = n;
        while n & 3 == 0 {
            n >>= 2;
        }
        if n & 7 == 7 {
            return 4;
        }
        n = ncopy;
        for i in 1..=sqrtn {
            let k = n - i * i;
            let sqrtk = (f32::sqrt(k as f32)) as i32;
            if sqrtk * sqrtk == k {
                return 2;
            }
        }
        3
    }
}

/* */

#[inline(always)]
fn is_square(x: i32) -> bool {
    let root = (x as f32).sqrt() as i32;
    x == root * root
}

impl Solution {
    pub fn num_squares(mut n: i32) -> i32 {
        if is_square(n) {
            return 1;
        }

        while n & 3 == 0 {
            n >>= 2;
        }
        
        if (n + 1) % 8 == 0 {
            return 4;
        }

        for i in 1..=((n / 2) as f32).sqrt() as i32 {
            if (is_square(n - i * i)) {
                return 2;
            }
        }

        return 3;
    }
}

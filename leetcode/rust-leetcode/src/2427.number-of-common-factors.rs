impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        fn gcd_i32(x:i32, y:i32) -> i32 {
            if y == 0 {return x};
            return gcd_i32(y, x % y);
        }
        let mut gcd = gcd_i32(a, b);
        let mut ans:i32 = 1;
        for i in 1..gcd {
            if a % i == 0 && b % i == 0{
                ans += 1;
            }
        }
        ans
    }
}

impl Solution3 {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let factors: Vec<i32> = (1..(a.min(b) + 1))
            .filter(|v| (a % *v == 0 && b % *v == 0))
            .collect();
        factors.len() as i32
    }
}

impl Solution2 {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut x = a;
        let mut y = b;
        let gcd = {
            // Euclidian algorithm for greatest common divisor
            while x != y {
                if x > y {
                    x -= y
                } else {
                    y -= x
                }
            }
            x
        };
        ((1 as i32)..=gcd)
            .filter(|&f| a % f + b % f == (0 as i32))
            .count() as i32
    }
}

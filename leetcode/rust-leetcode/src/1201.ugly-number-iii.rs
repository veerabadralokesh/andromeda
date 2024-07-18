impl Solution {
    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        fn gcd(x: i64, y: i64) -> i64 {
            if y == 0 {return x}
            gcd(y, x % y)
        }
        fn lcm(x: i64, y: i64) -> i64 {
            (x * y) / gcd(x, y)
        }
        let (a, b, c, n) = (a as i64, b as i64, c as i64, n as i64);
        let lcm_abc = lcm(lcm(a, b), c);
        let lcm_ab = lcm(a, b);
        let lcm_ac = lcm(a, c);
        let lcm_bc = lcm(b, c);
        // println!("{lcm_abc} {lcm_ab} {lcm_ac} {lcm_bc}");
        let ugly_less_equal_n = |x: i64| -> i64 {
            // println!("{} {} {} {} {} {} {}", x / a , x / b , x / c , x / lcm_ab , x/lcm_bc , x/lcm_ac , x / lcm_abc);
            (x/a) + (x/b) + (x/c) - (x/lcm_ab) - (x/lcm_bc) - (x/lcm_ac) + (x/lcm_abc)
        };
        // println!("uglies {:?}", ugly_less_equal_n(12));
        let (mut left, mut right, mut mid) = (0, i64::MAX, 0);
        // let mut count = 0;
        while left < right {
            mid = left + ((right - left)/2);
            if ugly_less_equal_n(mid) < n {
                left = mid+1;
            } else {
                right = mid;
            }
            // println!("{left} {mid} {right}");
            // count += 1;
            // if count == 100 {
            //     break;
            // }
        }
        left as _
    }
}



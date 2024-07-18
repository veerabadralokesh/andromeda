static MODULO:i64 = 1_000_000_007;
impl Solution {
    pub fn string_count(n: i32) -> i32 {
        if n < 4 {
            return 0;
        }
        fn pow_mod(x: i64, y: i64) -> i64 {
            if y == 0 {
                return 1;
            }
            if y == 1 {
                return x;
            }
            let x_pow_y_by_2 = pow_mod(x, y/2);
            if y & 1 == 0 {
                (x_pow_y_by_2 * x_pow_y_by_2) % MODULO
            } else {
                (x * ((x_pow_y_by_2 * x_pow_y_by_2) % MODULO)) % MODULO
            }
        }
        fn get_powers(x: i64, y: i64) -> (i64, i64) {
            let x_y_1 = pow_mod(x, y-1);
            ((x_y_1 * x) % MODULO, x_y_1)
        }
        let n = n as i64;
        let all_strings = pow_mod(26, n);
        let (p25_n, p25_n_1) = get_powers(25, n);
        let (p24_n, p24_n_1) = get_powers(24, n);
        let (p23_n, p23_n_1) = get_powers(23, n);
        let l = p25_n;
        let t = p25_n;
        let e = (p25_n + n * p25_n_1);
        let le = (p24_n + n * p24_n_1);
        let et = (p24_n + n * p24_n_1);
        let lt = p24_n;
        let elt = (p23_n + n * p23_n_1);
        let mut ans = all_strings - l - t - e + le + et + lt - elt;
        while ans < 0 {
            ans += MODULO;
        }
        (ans % MODULO) as _
    }
}


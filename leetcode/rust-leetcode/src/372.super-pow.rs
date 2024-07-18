const k_mod:i32 = 1337;
impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let mut ans = 1i32;
        let a = a % k_mod;
        fn mod_pow(x: i32, n: i32) -> i32 {
            if n == 0 {
                return 1;
            } else if n % 2 == 1 {
                return (x * mod_pow(x % k_mod, n-1)) % k_mod;
            }
            mod_pow((x * x) % k_mod, n/2) % k_mod
        }
        for i in b {
            ans = (mod_pow(ans, 10) * mod_pow(a, i)) % k_mod;
        }
        ans
    }
}


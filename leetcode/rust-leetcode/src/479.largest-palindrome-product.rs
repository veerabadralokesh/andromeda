impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9
        }
        let k_mod = 1337;
        let n = n as u64;
        let high = 10_u64.pow(n as u32);
        let low = 10_u64.pow(n as u32 - 1);

        let get_palindrome = |mut x: u64| -> u64 {
            let mut p = x;
            while x > 0 {
                p = p * 10 + x % 10;
                x /= 10;
            }
            p
        };
        
        for x in (low..high).rev() {
            let palindrome = get_palindrome(x);
            let mut y = high;
            while y * y >= palindrome {
                if palindrome % y == 0 {
                    return (palindrome % k_mod) as _;
                }
                y -= 1;
            }
        }
        -1
    }
}


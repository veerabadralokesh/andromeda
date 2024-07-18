impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {return 1;}
        let mut ans = 10i32;
        for i in 1..n {
            let mut ic = 9;
            let mut combos = 9;
            for _ in 0..i {
                ic *= combos;
                combos -= 1;
            }
            ans += ic;
        }
        ans
    }
}


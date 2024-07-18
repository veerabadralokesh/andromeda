impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut s:i32 = 0;
        let mut p:i32 = 1;
        let mut n:i32 = n;
        while n > 0 {
            let x:i32 = n % 10;
            s += x;
            p *= x;
            n = n/10;
        }
        p - s
    }
}

impl Solution2 {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let (sum ,  mult) = n.to_string().chars()
        .map(|c| c.to_digit(10).unwrap())
        .fold((0,1), |(sum,mult), c| {
            (sum + c ,mult * c)
        });
        (mult - sum) as i32
    }
}

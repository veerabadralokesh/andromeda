impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let (mut fbn2, mut fbn1, mut fbn) = (0, 1, 1);
        for _ in 3..=n {
           (fbn2, fbn1)  = (fbn1, fbn);
           fbn = fbn1 + fbn2;
        }
        fbn
    }
}

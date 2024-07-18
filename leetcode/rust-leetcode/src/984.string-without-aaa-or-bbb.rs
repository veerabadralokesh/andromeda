impl Solution {
    pub fn str_without3a3b(mut a: i32, mut b: i32) -> String {
        let (mut ac, mut bc) = (0, 0);
        let mut s = String::with_capacity((a+b) as usize);
        for _ in 0..(a+b) {
            if bc == 2 || (a > b && ac < 2) {
                s.push('a');
                a -= 1;
                bc = 0;
                ac += 1;
            } else {
                s.push('b');
                ac = 0;
                bc += 1;
                b -= 1;
            }
        }
        s
    }
}

impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        n.count_ones() as i32
    }
}

/*
*/

impl Solution2 {
    pub fn hammingWeight (n: u32) -> i32 {
        (0..32).fold(0, |acc, exp| acc + ((n & (1 << exp)) >> exp)) as i32
    }
}

/*
*/

impl Solution3 {
    pub fn hammingWeight (n: u32) -> i32 {
        let one = 1;
        let mut r = 0;
        for i in (0 .. 32) {
            if n & (one << i) != 0 {
                r += 1;
            }
        }
        return r;
    }
}

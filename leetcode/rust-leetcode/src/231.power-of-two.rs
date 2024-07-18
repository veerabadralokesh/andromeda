impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        let l = (n as f32).log(2.0);
        if l.is_nan(){
           return false;
        }
        2_i32.pow(l.round() as u32) == n
    }
}

impl Solution2 {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {return false}
        fn p2(n: i32) -> bool {
            if n == 1 {
                true
            } else if n%2 == 1 {
                false
            } else {
                p2(n / 2)
            }
        }
        p2(n)
    }
}

impl Solution3 {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        let mut mask = n;

        mask |= mask >> 1;
        mask |= mask >> 2;
        mask |= mask >> 4;
        mask |= mask >> 8;
        mask |= mask >> 16;
        mask = mask >> 1;
        
        (n & mask) == 0
    }
}

impl Solution4 {
    pub fn is_power_of_two(n: i32) -> bool {
        if n<=0{
            false
        }
        else if n&(n-1) == 0{
            true
        }
        else{
            false
        }
    }
}
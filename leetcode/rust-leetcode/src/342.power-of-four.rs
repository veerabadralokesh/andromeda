impl Solution {
    pub fn is_power_of_four(mut n: i32) -> bool {
        while (n > 0 && n & 3 == 0) {
            n >>=  2;
        }
        n == 1
    }
}

impl Solution2 {
    pub fn is_power_of_four(n: i32) -> bool {
        (n == 0x40000000) || (n == 0x10000000) || (n == 0x4000000) || (n == 0x1000000)
        || (n == 0x400000) || (n == 0x100000) || (n == 0x40000) || (n == 0x10000) 
        || (n == 0x4000) || (n == 0x1000) || (n == 0x400) || (n == 0x100) 
        || (n == 0x40) || (n == 0x10) || (n == 0x4) || (n == 0x1)
    }
}

impl Solution3 {
    pub fn is_power_of_four(n: i32) -> bool {
        let l = (n as f32).log(4.0);
        if l.is_nan(){
           return false;
        }
        4_i32.pow(l.round() as u32) == n
    }
}

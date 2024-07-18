impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        u32::reverse_bits(x)
    }
}

/*
*/

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
       let mut a:u32 =0; 
       let mut x =x;
       for _ in 0..31{
            a |=x&1;
            a<<=1;
            x>>=1;
       }
        a |=x&1; 
        a
    }
}

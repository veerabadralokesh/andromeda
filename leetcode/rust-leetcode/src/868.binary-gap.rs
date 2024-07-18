impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let mut pos = Vec::with_capacity(32);
        let mut i = 0;
        while n > 0 {
            if n & 1 == 1 {
                pos.push(i);
            }
            i += 1;
            n >>= 1;
        }
        if pos.len() == 1 {
            return 0;
        }
        i = 0;
        for j in 1..pos.len() {
            i = i.max(pos[j]-pos[j-1]);
        }
        i as i32
    }
}


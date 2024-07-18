impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        if left * right == 0 {return 0;}
        if left == right { return left; }
        let lb2 = format!("{:b}", left).into_bytes();
        let rb2 = format!("{:b}", right).into_bytes();
        // println!("{:?}", lb2);
        // println!("{:?}", rb2);
        if lb2.len() == rb2.len() {
            let mut ans:i32 = 1;
            let mut i:usize = 1;
            while i < lb2.len() {
                if lb2[i] == rb2[i] {
                    ans <<= 1;
                    ans += ((lb2[i] - b'0') as i32);
                    i += 1;
                } else {
                    ans <<= ((lb2.len() - i) as i32);
                    i = lb2.len();
                }
            }
            return ans;
        }
        0
    }
}

impl Solution3 {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        if left * right == 0 {return 0;}
        if left == right { return left; }
        let mut left = left;
        let mut right = right;
        let mut count:i32 = 0;
        while (left>0 && right>0) {
            if (left == right) {
                left <<= count;
                return left;
            } else {
                left >>= 1;
                right >>= 1;
                count += 1;
            }
        }
        0
    }
}

impl Solution2 {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        if left == right {
            return left;
        }

        let mut bit = 1 << 30;
        let mut result = 0;
        for _ in 0..31 {
            if bit & left != bit & right {
                return result;
            }

            if bit & left == bit {
                result |= bit;
            }
            bit >>= 1;
        }

        result
    }
}
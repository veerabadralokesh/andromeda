impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans:Vec<i32> = Vec::with_capacity((n+1) as usize);
        for j in 0..(n+1) {
            let mut i = j as i32;
            let mut count:i32 = 0;
            while i > 0 {
                count += (i & 1);
                i >>= 1;
            }
            ans.push(count);
        }
        ans
    }
}

impl Solution2 {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![0; n as usize + 1];
        for i in 1..=n {
            res[i as usize] = (res[i as usize >> 1] + (i & 1));
        }
        res
    }
}

impl Solution3 {
    pub fn count_bits(n: i32) -> Vec<i32> {
        (0..=n).map(|i| i.count_ones() as i32).collect()
    }
}
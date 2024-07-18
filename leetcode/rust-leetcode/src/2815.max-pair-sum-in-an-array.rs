use std::collections::HashMap;
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::with_capacity(10);
        let max_num = |mut n: i32| -> i32 {
            let mut mx = 0;
            while n > 0 {
                mx = mx.max(n % 10);
                n /= 10;
            }
            mx
        };
        for n in nums {
            map.entry(max_num(n)).or_insert(Vec::new()).push(n);
        }
        let mut ans = -1;
        let (mut max1, mut max2) = (0, 0);
        for (k, v) in map.into_iter() {
            if v.len() > 1 {
                (max1, max2) = (0, 0);
                for &n in v.iter() {
                    if n > max1 {
                        max2 = max1;
                        max1 = n;
                    } else if n > max2 {
                        max2 = n;
                    }
                }
                ans = ans.max(max1 + max2);
            }
        }
        ans
    }
}


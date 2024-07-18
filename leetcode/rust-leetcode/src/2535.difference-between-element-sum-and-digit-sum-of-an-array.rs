use std::collections::HashMap;
impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut sum:i32 = 0;
        let mut map = HashMap::new();
        for n in nums {
            if n > 9 {
                if !map.contains_key(&n) {
                    let mut x:i32 = n;
                    let mut y:i32 = n;
                    while x > 0 {
                        y -= x %10;
                        x /= 10;
                    }
                    map.insert(n, y);
                    sum += y;
                } else {
                    sum += map.get(&n).unwrap();
                }
            }
        }
        sum
    }
}


fn sum_of_digits(mut num: i32) -> i32 {
    let mut res = 0;
    while num > 0 {
        res += num % 10;
        num /= 10;
    }
    res
}

impl Solution2 {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        nums.into_iter().map(|x| x - sum_of_digits(x)).sum()
    }
}


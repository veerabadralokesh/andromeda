use std::collections::HashMap;
impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        if high < 11 || (low > 99 && high < 1001) {
            return 0;
        }
        let mut search_ranges = vec![];
        if low < 100 {
            search_ranges.push(vec![low.max(11) as usize, high.min(99) as usize + 1]);
        }
        if high > 1000 {
            search_ranges.push(vec![low.max(1001) as usize, high.min(9999) as usize + 1]);
        }
        let mut map = HashMap::new();
        let mut get_sum = |n: &usize| -> usize {
            if !map.contains_key(n) {
                let mut x = *n;
                let mut sum = 0;
                while x > 0 {
                    sum += x % 10;
                    x = x/10;
                }
                map.insert(*n, sum);
            }
            *map.get(n).unwrap()
        };
        let mut is_symmetric = |n: &usize| -> bool {
            let lhs = n / 100;
            let rhs = n % 100;
            get_sum(&lhs) == get_sum(&rhs)
        };
        let mut ans = 0;
        for range in search_ranges {
            if range[1] < 101 {
                for x in range[0]..range[1] {
                    if x % 11 == 0 {
                        ans += 1;
                    }
                }
            } else {
                for x in range[0]..range[1] {
                    if is_symmetric(&x) {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}


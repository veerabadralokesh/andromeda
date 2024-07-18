impl Solution {
    pub fn max_score(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;
        nums.sort_by_key(|&n| -n);
        let mut prefix = 0;
        for n in nums {
            prefix += n as i64;
            if prefix <= 0 {
                return count;
            } else {
                count += 1;
            }
        }
        count
    }
}

/* */

impl Solution {
    pub fn max_score(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;
        nums.sort_unstable_by_key(|&n| -n);
        let mut prefix = 0i64;
        for n in nums {
            prefix += n as i64;
            if prefix <= 0 {
                return count;
            } else {
                count += 1;
            }
        }
        count
    }
}

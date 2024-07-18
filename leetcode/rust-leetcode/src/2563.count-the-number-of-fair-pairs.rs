impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort();
        let mut ans = 0i64;
        for (i,&n) in nums.iter().enumerate() {
            if i == nums.len() {break;}
            let lb = match nums[i+1..].binary_search(&(lower-n)) {
                Ok(idx) => {
                    let mut idx = idx;
                    while idx > 0 && nums[idx+i] == nums[idx+i+1] {
                        idx -= 1;
                    }
                    idx
                },
                Err(idx) => {
                    idx
                }
            };
            let ub = match nums[i+1..].binary_search(&(upper-n)) {
                Ok(idx) => {
                    let mut idx = idx;
                    while idx+i+1 < nums.len()-1 && nums[idx+i+1] == nums[idx+i+2] {
                        idx += 1;
                    }
                    idx+1
                },
                Err(idx) => {
                    if idx > 0 {idx} else {continue}
                }
            };
            ans += (ub - lb) as i64;
        }
        ans
    }
}

/* */

// LEARN

impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums; nums.sort();
        let r = 1+(1..nums.len()).collect::<Vec<_>>().partition_point(|&i| {nums[i-1]+nums[i]<lower});
        if r==nums.len() {return 0;}
        let (mut ll, mut lr) = (nums[..r].partition_point(|&x| x+nums[r]<lower),nums[..r].partition_point(|&x| x+nums[r]<=upper));
        let mut acc = lr-ll;
        for r in 1+r..nums.len() {
            let nr = nums[r];
            ll = nums[..=ll].partition_point(|&x| x+nr<lower);
            lr = nums[..r].partition_point(|&x| x+nr<=upper);
            match (nums[ll]+nr).cmp(&upper) {
                std::cmp::Ordering::Greater=> break,
                _=>acc+=lr-ll
            }
        }
        acc as i64
    }
}



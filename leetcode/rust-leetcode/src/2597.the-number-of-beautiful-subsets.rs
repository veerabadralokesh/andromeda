impl Solution {
    pub fn beautiful_subsets(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = [0; 1002];
        let mut flags = [true; 1002];
        let l = nums.len();
        nums.sort();
        let nums = nums.iter().map(|&n| n as usize).collect::<Vec<_>>();
        for &n in nums.iter() {
            counts[n] += 1;
        }
        let k = k as usize;
        fn backtrack(nums: &Vec<usize>, k: usize, flags: &mut [bool], i: usize, ans: &mut i32, count: i32, l: usize) {
            if l == i {
                if count > 0 {
                    *ans += 1;
                }
                return;
            }
            backtrack(nums, k, flags, i+1, ans, count, l);
            let idx = nums[i];
            if flags[idx] {
                let nidx = (idx + k).min(1001);
                flags[nidx] = false;
                backtrack(nums, k, flags, i+1, ans, count+1, l);
                flags[nidx] = true;
            }
        }
        let mut ans = 0;
        backtrack(&nums, k, &mut flags, 0, &mut ans, 0, l);
        ans
    }
}

/* */

// DOESNT WORK

impl Solution {
    pub fn beautiful_subsets(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = [0; 1001];
        let mut flags = [false; 1001];
        let l = nums.len() as u32;
        let mut max = 0;
        for &n in nums.iter() {
            counts[n as usize] += 1;
            max = max.max(n);
        }
        let k = k as usize;
        let mut total = 2_i32.pow(l) - 1;
        for i in k..=max as usize {
            if counts[i] > 0 && counts[i-k] > 0 {
                let m = counts[i] as u32;
                let n = counts[i-k] as u32;
                let z = l - m - n;
                println!("{i} {} {m} {n} {z}", (i-k));
                total -= ((m * n) as i32 * 2_i32.pow(z));
                // if flags[i-k] {
                //     total += 1;
                // }
                flags[i] = true;
                flags[i-k] = true;
            }
        }
        total
    }
}


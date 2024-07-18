use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set = nums.iter().collect::<HashSet<_>>();
        let mut ans = 0;
        for &num in &set {
            if !set.contains(&(num-1)) {
                let mut count = 1;
                let mut i = num + 1;
                while set.contains(&i) {
                    count += 1;
                    i += 1;
                }
                ans = ans.max(count);
            }
        }
        ans
    }
}

/* */

// LEARN

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<_> = nums.into_iter().collect();
        let mut ans = 0;
        for &num in &num_set {
            if !num_set.contains(&(num - 1)) {
                let count = (num..).take_while(|x| num_set.contains(x)).count();
                ans = ans.max(count);
            }
        }
        ans as i32
    }
}

/* */

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: std::collections::HashSet<_> = nums.iter().collect();
        set.iter()
            .filter(|&&x| !set.contains(&(x - 1))) // ensure not part of a longer consecutive sequence
            .map(|&&x| (x..).take_while(|x| set.contains(x)).count())
            .max()
            .unwrap_or(0) as _
    }
}

/* */

impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        Self::radix_sort_base10(&mut nums);
        nums.iter()
            .zip(nums.iter().skip(1))
            .scan(1, |n, (&a, &b)| {
                *n = if a + 1 >= b { *n + b - a } else { 1 }; // +1 length if (a+1 == b), +0 if (a == b); otherwise reset length
                Some(*n)
            })
            .max()
            .unwrap_or(nums.len().min(1) as _)
    }

    fn radix_sort_base10(nums: &mut [i32]) {
        let mut buckets = vec![vec![]; 20]; // for digits -9 to +9
        for i in 0..10 {
            nums.iter()
                .for_each(|&x| buckets[10 + ((x / 10i32.pow(i)) % 10) as usize].push(x));
            buckets
                .iter()
                .flat_map(|b| b.iter())
                .zip(nums.iter_mut())
                .for_each(|(&x, y)| *y = x);
            buckets.iter_mut().for_each(|b| b.clear());
        }
    }
}

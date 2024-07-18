impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0usize;
        let mut j = numbers.len() - 1;
        while i < j {
            let sum = numbers[i] + numbers[j];
            if sum < target {
                i += 1;
            } else if sum > target {
                j -= 1;
            } else {
                break;
            }
        }
        vec![i as i32 + 1,j as i32 + 1]
    }
}

/*
*/

use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hi = match numbers.binary_search(&(target - numbers[0])) {
            Ok(x) => return vec![1, x as i32 + 1],
            Err(x) => x - 1,
        };
        let mut lo = if hi == numbers.len() - 1 {
            match numbers[..hi].binary_search(&(target - numbers[hi])) {
                Ok(x) => return vec![x as i32 + 1, hi as i32 + 1],
                Err(x) => x,
            }
        }
        else {
            1
        };
        loop {
            match (numbers[lo] + numbers[hi]).cmp(&target) {
                Ordering::Equal => return vec![lo as i32 + 1, hi as i32 + 1],
                Ordering::Less => lo += 1,
                Ordering::Greater => hi -= 1,
            }
        }
    }
}

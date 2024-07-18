use std::collections::HashSet;
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut perms:HashSet<Vec<i32>> = HashSet::with_capacity(match nums.len() {
            1 => 1,
            2 => 2,
            3 => 6,
            4 => 24,
            5 => 120,
            6 => 720,
            7 => 5040,
            _ => 40320
        });
        Solution::generate(nums.len(), &mut nums, &mut perms);
        perms.into_iter().collect()
    }
    fn generate(mut k: usize, nums: &mut Vec<i32>, perms: &mut HashSet<Vec<i32>>) {
        if k == 1 {
            perms.insert(nums.clone());
        } else {
            k -= 1;
            Solution::generate(k, nums, perms);
            for i in 0..k {
                if k & 1 == 1 {
                    nums.swap(i, k);
                } else {
                    nums.swap(0, k);
                }
                Solution::generate(k, nums, perms);
            }
        }
    }
}

/* */

// LEARN

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let Some(a) = nums.pop() else { return vec![vec![]]; };
        Solution::permute_unique(nums).into_iter().flat_map(|permutation| {
            (0..=permutation.iter().position(|&v| v == a).unwrap_or(permutation.len())).map(move |i| {
                let mut p = permutation.clone();
                p.insert(i, a);
                p
            })
        }).collect()
    }
}

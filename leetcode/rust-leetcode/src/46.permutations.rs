impl Solution {
    pub fn gen(ans: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>) {
        let n = nums.len();
        if n == 1 {
            ans.push(nums.to_vec());
            return;
        }
        for i in 0..n {
            let mut subans:Vec<Vec<i32>> = Vec::with_capacity(n);
            let num = nums[i];
            let mut subnums = nums.clone();
            subnums.remove(i);
            // println!("{n}, {num}, {:?}", subnums);
            Self::gen(&mut subans, &mut subnums);
            // println!("{:?}", subans);
            for j in 0..subans.len() {
                subans[j].push(num);
                ans.push(subans[j].to_vec());
            }
        }
    }

    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut ans:Vec<Vec<i32>> = Vec::with_capacity((n+1)*n/2);
        Self::gen(&mut ans, &mut nums);
        ans
    }
}

/* */

// LEARN

impl Solution {
    pub 
fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut perms = Vec::with_capacity(match nums.len() {
        1 => 1,
        2 => 2,
        3 => 6,
        4 => 24,
        5 => 120,
        _ => 720,
    });
    Solution::generate(nums.len(), &mut nums, &mut perms);
    perms
}

fn generate(mut k: usize, nums: &mut Vec<i32>, perms: &mut Vec<Vec<i32>>) {
    if k == 1 {
        perms.push(nums.clone());
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

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let Some(a) = nums.pop() else { return vec![vec![]]; };
        Solution::permute(nums).into_iter().flat_map(|permutation| {
            (0..=permutation.iter().position(|&v| v == a).unwrap_or(permutation.len())).map(move |i| {
                let mut p = permutation.clone();
                p.insert(i, a);
                p
            })
        }).collect()
    }
}

/* */

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(nums: &mut Vec<i32>, start: usize, res: &mut Vec<Vec<i32>>) {
            if start == nums.len() {
                res.push(nums.clone());
                return;
            }
            for i in start..nums.len() {
                nums.swap(start,i);
                backtrack(nums, start + 1, res);
                nums.swap(start, i);
            }
        }
        let mut nums = nums;
        let mut res = Vec::new();
        backtrack(&mut nums, 0, &mut res);
        res
    }
}

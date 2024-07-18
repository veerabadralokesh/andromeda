impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return Vec::new();
        }
        let target = target as i64;
        let mut nums = nums.iter().map(|&n| n as i64).collect::<Vec<_>>();
        nums.sort();
        let mut ans = Vec::new();
        for i in 0..nums.len() - 3 {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }
            for j in i+1..nums.len() - 2 {
                if j > i+1 && nums[j] == nums[j-1] {
                    continue;
                }
                let ij = nums[i] + nums[j];
                for k in j+1..nums.len() - 1 {
                    if k > j+1 && nums[k] == nums[k-1] {
                        continue;
                    }
                    let ijk = ij + nums[k];
                    for l in k+1..nums.len() {
                        if l > k+1 && nums[l] == nums[l-1] {
                            continue;
                        }
                        let ijkl = ijk + nums[l];
                        if ijkl == target {
                            ans.push(vec![
                                nums[i] as i32,
                                nums[j] as i32,
                                nums[k] as i32,
                                nums[l] as i32
                                ]);
                            break;
                        }
                    }
                }
            }
        }
        ans
    }
}

/* */

// LEARN

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn k_sum(nums: Vec<i64>, target: i64, k: i64) -> Vec<Vec<i32>> {
            fn two_sum(nums: Vec<i64>, target: i64) -> Vec<Vec<i32>> {
                let mut res = Vec::new();
                // let mut set = HashSet::new();
                let mut left = 0;
                let mut right = nums.len() - 1;
                while left < right {
                    let sum: i64 = (nums[left] + nums[right]) as i64;
                    if sum < target || (left > 0 && nums[left] == nums[left - 1]) {
                        left += 1;
                    } else if sum > target || (right < nums.len() - 1 && nums[right] == nums[right + 1]) {
                        right -= 1;
                    } else {
                        res.push(vec![nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                    }
                }
                // for v in set {
                //     res.push(v);
                // }
                res.into_iter().map(|x| x.into_iter().map(|x| x as i32).collect()).collect()
            }
            let mut res = Vec::new();
            let average: i64 = (target / k) as i64;
            if nums.is_empty() ||
                nums[0] as i64 * (k) as i64 > target as i64 ||
                *nums.last().unwrap() as i64 * (k as i64) < target as i64
            {
                return res;
            }
            if k == 2 {
                return two_sum(nums, target);
            }
            for i in 0..nums.len() {
                if i == 0 || nums[i - 1] != nums[i] {
                    for mut set in k_sum(nums[i + 1..].to_vec(), target - nums[i], k - 1) {
                        set.push(nums[i].try_into().unwrap());
                        res.push(set);
                    }
                }
            }
            res
        }
        let mut nums: Vec<i64> = nums.clone().into_iter().map(|x| x as i64).collect();
        nums.sort();
        return k_sum(nums, target as i64, 4);
    }
}

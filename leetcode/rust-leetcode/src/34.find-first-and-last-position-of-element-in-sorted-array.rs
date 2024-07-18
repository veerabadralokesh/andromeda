impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {return vec![-1,-1];}
        let mut start = nums.partition_point(|x| x < &target) as i32;
        let mut end = nums.partition_point(|x| x <= &target) as i32;
        if start >= nums.len() as i32 || nums[start as usize] != target {start = -1; end = -1;}
        else {end -= 1;}
        vec![start,end]
    }
}

/* */

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
                let n = nums.len();
        match nums.binary_search(&target) {
            //if there are multiple matches, then any one of the matches could be returned
            Ok(i) => {
                let mut l = i;
                let mut r = i;
                //sorted in non-decreasing order...
                while l > 0 && nums[l - 1] == target {
                    l -= 1;
                }
                while r + 1 < n && nums[r + 1] == target {
                    r += 1;
                }
                vec![l as i32, r as i32]
            }
            Err(_) => vec![-1, -1],
        }
    }
}

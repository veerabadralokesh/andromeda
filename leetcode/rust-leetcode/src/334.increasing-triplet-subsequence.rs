impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut i:i32 = i32::MAX;
        let mut j:i32 = i32::MAX;
        for n in nums {
            if i >= n {
                i = n;
            } else if j >= n {
                j = n;
            } else {
                return true;
            }
        }
        false
    }
}


impl Solution2 {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 { return false; }
        for i in 0..nums.len()-2 {
            for j in i+1..nums.len()-1 {
                if nums[j] <= nums[i] {
                    continue;
                }
                for k in j+1..nums.len() {
                    if nums[k] <= nums[j] {
                        continue;
                    }
                    if nums[k] > nums[j] && nums[j] > nums[i] {
                        return true;
                    }
                }
            }
        }
        false
    }
}
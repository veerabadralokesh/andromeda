impl Solution {
    pub fn unequal_triplets(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let l = nums.len();
        let mut ans = 0;
        for i in 0..l-2 {
            for j in i+1..l-1 {
                if nums[i] == nums[j] {
                    continue;
                }
                for k in j+1..l {
                    if nums[j] != nums[k] {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}


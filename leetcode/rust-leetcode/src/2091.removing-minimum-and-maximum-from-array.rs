impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let (mut mx, mut mn, mut mxi, mut mni, l) = (nums[0], nums[0], 0, 0, nums.len());
        for (i, &n) in nums.iter().enumerate() {
            if mx < n {
                mxi = i;
                mx = n;
            }
            if mn > n {
                mni = i;
                mn = n;
            }
        }
        // println!("{mxi} {mni} {l}");
        l.min(mxi.max(mni) + 1).min(l - mxi.min(mni)).min(mni.min(mxi)+1+l-mni.max(mxi)) as _
    }
}


impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let l = nums.len();
        if nums[0] >= l as i32 {
            return l as _;
        }
        for i in 1..l {
            let count = (l - i) as i32;
            if nums[i-1] < count && nums[i] >= count {
                return count;
            }
        }
        -1
    }
}


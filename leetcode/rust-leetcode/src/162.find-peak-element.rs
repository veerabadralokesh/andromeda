impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 { return 0; }
        if nums[0] > nums[1] { return 0 ;}
        if nums[n-1] > nums[n-2] {return (n - 1) as i32;}
        let (mut lo, mut hi) = (1usize, n-2);
        while lo <= hi {
            let mid = lo + (hi - lo)/2;
            if nums[mid] > nums[mid - 1] {
                if nums[mid] > nums[mid + 1] {
                    return mid as i32;
                } else {
                    lo = mid + 1;
                }
            } else {
                hi = mid - 1;
            }
        }
        -1
    }
}

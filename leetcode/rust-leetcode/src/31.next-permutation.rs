impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let l = nums.len();
        let mut i = (l-2) as i32;
        while i > -1 {
            if nums[i as usize] < nums[(i+1) as usize] {
                break;
            }
            i -= 1;
        }
        if i > -1 {
            let i = i as usize;
            for j in (i+1..l).rev() {
                if nums[j] > nums[i] {
                    nums.swap(i, j);
                    break;
                }
            }
            nums[i+1..].reverse();
        } else {
            nums.reverse();
        }
    }
}


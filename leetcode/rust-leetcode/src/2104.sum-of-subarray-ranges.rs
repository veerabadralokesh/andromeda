impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let nums = nums.iter().map(|&n| n as i64).collect::<Vec<_>>();
        let mut stackl = Vec::with_capacity(nums.len());
        let mut stacks = Vec::with_capacity(nums.len());
        let (mut largest, mut smallest, mut i, l) = (0, 0, 0, nums.len() as i64);
        while i <= l {
            while !stackl.is_empty() && (i == l || nums[*stackl.last().unwrap() as usize] <= nums[i as usize]) {
                if let Some(j) = stackl.pop() {
                    largest += (nums[j as usize] * (i-j) * (j-stackl.last().unwrap_or(&-1)));
                }
            }
            while !stacks.is_empty() && (i == l || nums[*stacks.last().unwrap() as usize] >= nums[i as usize]) {
                if let Some(j) = stacks.pop() {
                    smallest += (nums[j as usize] * (i-j) as i64 * (j-stacks.last().unwrap_or(&-1)));
                }
            }
            if i < l {
                stackl.push(i);
                stacks.push(i);
            }
            i += 1;
        }
        largest - smallest
    }
}


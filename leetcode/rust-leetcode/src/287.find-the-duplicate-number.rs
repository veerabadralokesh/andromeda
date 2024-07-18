impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // let nums = nums.iter().map(|x| x as usize).collect::<Vec<_>>();
        let mut counts = vec![0; nums.len()+2];
        for n in nums {
            if counts[n as usize] == 1 {
                return n;
            }
            counts[n as usize] += 1;
        }
        0
    }
}

/* */

impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        let mut idx = 0usize;
        loop {
            let temp = nums[idx];
            if temp == -1 {
                return idx as i32;
            }
            nums[idx] = -1;
            idx = temp as usize;
        }
        0
    }
}

/* */

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // Solving it similar to a linked list with a cycle problem
        let nums = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let mut slow = nums[nums[0]];
        let mut fast = nums[slow];
        while slow != fast {
            slow = nums[slow];
            fast = nums[nums[fast]];
        }
        slow = nums[0];
        while slow != fast {
            slow = nums[slow];
            fast = nums[fast];
        }
        slow as i32
    }
}

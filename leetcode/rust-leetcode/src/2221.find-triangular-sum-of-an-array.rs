impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        for i in (1..nums.len()).rev() {
            for j in 0..i {
                nums[j] = (nums[j] + nums[j+1]) % 10;
            }
        }
        nums[0]
    }
}


/* */

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        for index in (0..nums.len()).rev() {
            for index in 0..index {
                nums[index] += nums[index + 1];
            }

            if index % 23 == 0 {
                nums.iter_mut().for_each(|x| *x %= 10);
            }
        }

        nums[0] % 10
    }
}

/* */

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        for i in (1..nums.len()).rev() {
            for j in 0..i {
                nums[j] += nums[j+1];// % 10;
            }
            if i % 23 == 0 {
                nums.iter_mut().for_each(|x| *x %= 10);
            }
        }
        nums[0] % 10
    }
}

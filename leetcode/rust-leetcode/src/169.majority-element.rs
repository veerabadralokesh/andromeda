impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut majority_num: i32 = nums[0];
        let mut count: i32 = 0;
        for num in nums {
            // println!("{},{}", num, majority_num);
            if num == majority_num {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                majority_num = num;
                count = 1;
            }
        }
        return majority_num;
    }
}
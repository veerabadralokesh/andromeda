impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut rnums = nums.to_vec();
        let mut pi: usize = 0;
        let mut ni: usize = 1;
        // let mut neg: bool = false;
        for i in 0..nums.len() {
            let num = nums[i];
            // println!("{}", num);
            if num >= 0 {
                rnums[pi] = num;
                pi += 2;
            } else {
                rnums[ni] = num;
                ni += 2;
            }
        }
        rnums
    }
}


impl Solution2 {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut positive = vec![];
    let mut negative: Vec<i32> = vec![];
    let mut result: Vec<i32> = vec![];
    for num in nums {
        if num < 0 {
            negative.push(num);
        } else {
            positive.push(num);
        }
    }

    let mut i = 0;

    while i < positive.len() {
        result.push(positive[i]);
        result.push(negative[i]);
        i = i + 1;
    }

    return result;
    }
}

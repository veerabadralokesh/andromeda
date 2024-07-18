impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut count:i32 = 0;
        for i in 0..nums.len()-2 {
            for j in i+1..nums.len()-1 {
                if nums[j] - nums[i] > diff {
                    break;
                }
                if nums[j] - nums[i] == diff {
                    for k in j+1..nums.len() {
                        if nums[k] - nums[j] > diff {
                            break;
                        }
                        if nums[k] - nums[j] == diff {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}
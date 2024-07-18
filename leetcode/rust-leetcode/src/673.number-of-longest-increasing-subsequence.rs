impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut length = vec![1; n];
        let mut count = vec![1; n];
        for (i, &num) in nums.iter().enumerate() {
            for j in 0..i {
                if nums[j] < num {
                    if length[i] < length[j] + 1 {
                        length[i] = length[j] + 1;
                        count[i] = count[j];
                    } else if length[i] == length[j] + 1 {
                        count[i] += count[j];
                    }
                }
            }
        }
        // println!("{:?}", length);
        // println!("{:?}", count);
        let (mut ans, mut max_len) = (0, 0);
        for i in 0..n {
            if length[i] > max_len {
                max_len = length[i];
                ans = count[i];
            } else if length[i] == max_len {
                ans += count[i];
            }
        }
        ans
    }
}


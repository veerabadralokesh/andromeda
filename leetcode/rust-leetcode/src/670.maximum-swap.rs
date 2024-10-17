impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut nums = num.to_string().bytes().map(|b| (b - b'0') as usize).collect::<Vec<_>>();
        // println!("{:?}", nums);
        let mut lastIdx = [-1; 10];
        for (i, &n) in nums.iter().enumerate() {
            lastIdx[n] = i as i32;
        }
        for i in 0..nums.len() as i32 {
            for j in (nums[i as usize]+1..10).rev() {
                if lastIdx[j] > i {
                    nums.swap(lastIdx[j] as usize, i as usize);
                    // println!("{:?}", nums);
                    return nums.iter().map(|&b| (b as u8 + b'0') as char).collect::<String>().parse::<i32>().unwrap();
                }
            }
        }
        num
    }
}


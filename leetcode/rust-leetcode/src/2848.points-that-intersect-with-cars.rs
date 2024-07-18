// use std::collections::HashMap;
impl Solution {
    pub fn number_of_points(mut nums: Vec<Vec<i32>>) -> i32 {
        nums.sort();
        let mut stack:Vec<Vec<i32>> = Vec::new();
        let mut i = 0;
        stack.push(nums[0].to_vec());
        for num in nums.iter().skip(1) {
            if stack[i][1] >= num[0] {
                stack[i][1] = stack[i][1].max(num[1]);
            } else {
                stack.push(num.to_vec());
                i += 1;
            }
        }
        let mut ans = 0;
        for num in stack {
            ans += num[1] - num[0] + 1;
        }
        ans
    }
}

/* */

// use std::collections::HashMap;
impl Solution {
    pub fn number_of_points(mut nums: Vec<Vec<i32>>) -> i32 {
        let mut track = [0; 102];
        for num in nums {
            track[num[0] as usize] += 1;
            track[(num[1]+1) as usize] -= 1;
        }
        let mut ans = 0;
        let mut start = None;
        let mut count = 0;
        for i in 0..track.len() {
            if track[i] > 0 {
                count += track[i];
                if start.is_none() {
                    start = Some(i as i32);
                }
            } else if track[i] < 0 {
                count += track[i];
                if count == 0 {
                    ans += (i as i32) - start.unwrap();
                    start = None;
                }
            }
        }
        // println!("{:?}", track);
        ans
    }
}

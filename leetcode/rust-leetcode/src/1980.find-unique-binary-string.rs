impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums[0].len();
        let mut nnums = nums.iter().map(|x| i32::from_str_radix(x, 2).unwrap()).collect::<Vec<_>>();
        nnums.sort();
        let mut i = 0i32;
        for n in nnums {
            if i != n {
                break;
            }
            i += 1;
        }
        // let temp = format!("{i:b}");
        // let mut ans = String::new();
        // for _ in 0..(n-temp.len()) {
        //     ans.push('0');
        // }
        // ans.push_str(temp.as_str());
        // ans
        format!("{i:0n$b}")
    }
}

/*
*/

use std::collections::HashSet;
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let values: HashSet<_> = nums.iter().map(|s| u16::from_str_radix(s, 2).unwrap()).collect();
        let ans = (0..u16::MAX).find(|num| !values.contains(num)).unwrap();
        let n = nums.len();
        format!("{ans:0n$b}")
    }
}

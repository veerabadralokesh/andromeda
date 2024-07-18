impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let nums = num.to_string().into_bytes().iter().map(|&n| (n-b'0') & 1).collect::<Vec<_>>();
        println!("{:?}", nums);
        let mut i = nums.len() as i32 - 1;
        while i > -1 {
            if nums[i as usize] == 1 {
                break;
            }
            i -= 1;
        }
        num[..((i + 1) as usize)].to_string()
    }
}

/* */

impl Solution {
    pub fn largest_odd_number(mut num: String) -> String {
        while let Some(d) = num.pop() {
            if d.to_digit(10).unwrap() & 1 == 1 {
                num.push(d);
                return num
            }
        }
        num
    }
}


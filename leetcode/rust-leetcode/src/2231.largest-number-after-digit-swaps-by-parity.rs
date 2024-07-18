impl Solution {
    pub fn largest_integer(mut num: i32) -> i32 {
        let mut nums = vec![];
        let mut even = vec![];
        let mut odd = vec![];
        while num > 0 {
            let x = num % 10;
            if x & 1 == 0 {
                even.push(x);
                nums.push(true);
            } else {
                odd.push(x);
                nums.push(false);
            }
            num /= 10;
        }
        nums.reverse();
        even.sort();
        even.reverse();
        odd.sort();
        odd.reverse();
        let (mut i, mut j) = (0, 0);
        let mut ans = 0;
        for k in 0..nums.len() {
            if nums[k] {
                ans = ans * 10 + even[i];
                i += 1;
            } else {
                ans = ans * 10 + odd[j];
                j += 1;
            }
        }
        ans
    }
}


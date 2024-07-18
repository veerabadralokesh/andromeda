impl Solution {
    pub fn next_greater_element(mut n: i32) -> i32 {
        let mut n = n as i64;
        let mut nums = vec![];
        while n > 0 {
            nums.push(n%10);
            n /= 10;
        }
        nums.reverse();
        let l = nums.len();
        let mut i = (l-2) as i64;
        while i > -1 {
            if nums[i as usize] < nums[(i+1) as usize] {
                break;
            }
            i -= 1;
        }
        if i > -1 {
            let i = i as usize;
            for j in (i+1..l).rev() {
                if nums[j] > nums[i] {
                    nums.swap(i, j);
                    break;
                }
            }
            nums[i+1..].reverse();
            for i in (0..l) {
                n = n * 10 + nums[i];
            }
            if n > i32::MAX as i64 {
                -1
            } else {
                n as i32
            }
        } else {
            -1
        }
    }
}

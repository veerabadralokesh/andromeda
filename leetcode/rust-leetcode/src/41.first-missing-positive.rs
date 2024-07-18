impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut temp:i32 = 0;
        let mut n:i32 = 0;
        for i in 0..l {
            n = nums[i];
            while true {
                if n > (l as i32) || n < 1 {
                    break;
                } else if n == nums[(n-1) as usize] {
                    break;
                } else if n > 0 {
                    temp = nums[(n-1) as usize];
                    nums[(n-1) as usize] = n;
                    n = temp;
                }
            }
        }
        for i in 0..l {
            if nums[i] != (i + 1) as i32 {
                return (i+1) as i32;
            }
        }
        (l+1) as i32
    }
}

/* */

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut i = 0usize;
        let mut prevn = 0;
        while i < l {
            if nums[i] > 0 {
                let n = nums[i] as usize;
                if n < l && (n-1) != i && n != prevn {
                    nums.swap(i, n-1);
                    prevn = n;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        // println!("{:?}", nums);
        for j in 1..l+1 {
            if nums[j-1] != (j as i32) {
                return j as i32;
            }
        }
        (l+1) as i32
    }
}

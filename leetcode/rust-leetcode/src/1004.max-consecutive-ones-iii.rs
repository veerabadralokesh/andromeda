impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut rp = 0usize;
        let mut lp = 0usize;
        let mut kr = k;
        let mut max_count = 0i32;
        let mut count = 0i32;
        while rp < nums.len() {
            if nums[rp] == 1 {
                count += 1;
                max_count = max_count.max(count);
                rp += 1;
            } else if kr > 0 {
                count += 1;
                max_count = max_count.max(count);
                kr -= 1;
                rp += 1;
            } else if k == 0 {
                rp += 1;
                count = 0;
            } else {
                while (k > 0 && kr == 0) && lp < rp {
                    match(nums[lp]) {
                        1 => {
                        },
                        _ => {
                            kr += 1;
                        }
                    }
                    lp += 1;
                    count -= 1;
                }
            }
            // println!("{lp} {rp} {count} {max_count} {kr}");
        }
        max_count
    }
}

/* */

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut max = 0;
        let mut count = 0;
        let mut left = 0;
        let mut right = 0;

        while right < nums.len() {
            if nums[right] == 0 {
                count += 1;
            }

            while count > k {
                if nums[left] == 0 {
                    count -= 1;
                }
                left += 1;
            }

            max = max.max(right - left + 1);
            right += 1;
        }

        max as i32
    }
}


impl Solution {
    pub fn min_number(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        let mut counts = [0; 10];
        let (mut min1, mut min2) = (10, 10);
        for n in nums1 {
            min1 = n.min(min1);
            counts[n as usize] += 1;
        }
        for n in nums2 {
            min2 = n.min(min2);
            counts[n as usize] += 1;
        }
        for i in 0..10 {
            if counts[i] == 2 {
                return i as i32;
            }
        }
        if min1 < min2 {
            min1 * 10 + min2
        } else {
            min2 * 10 + min1
        }
    }
}


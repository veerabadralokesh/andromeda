impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
        let l = nums.len();
        let mut nums2:Vec<i64> = Vec::with_capacity(nums.len());
        for n in nums.iter() {
            nums2.push(*n as i64);
        }
        nums2.sort();
        let mut sums = nums2.to_vec();
        // println!("{:?}", nums2);
        for i in 1..l {
            sums[i] = sums[i] + sums[i-1];
        }
        // println!("{:?}", sums);
        let mut p = sums[l-1];
        for i in 0..l {
            if (i == l - 2) {
                p = -1;
                break;
            }
            p = sums[l-i-1];
            if nums2[l-i-1] < sums[l-i-2] {
                break;
            }
        }
        p
    }
}

// [1,12,1,2,5,50,3]
// [1,12,1,2,5,50,3,23]

impl Solution2 {
	pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
		nums.sort_unstable();
		let mut s = nums.iter().map(|i| *i as i64).sum::<i64>();
		for &n in nums[2..].iter().rev() {
			let n = n as i64;
			s -= n;
			if s > n {
				return s + n;
			}
		}
		-1
	}
}

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut leftsum: Vec<i32> = Vec::new();
        let mut rightsum: Vec<i32> = Vec::new();
        leftsum.push(0);
        rightsum.push(0);
        let l = nums.len();
        for i in 1..l {
            leftsum.push(leftsum[i-1] + nums[i-1]);
            rightsum.push(rightsum[i-1] + nums[l-i]);
        }
        // println!("{:?}", leftsum);
        // println!("{:?}", rightsum);
        for i in 0..l {
            leftsum[i] = i32::abs(leftsum[i]-rightsum[l-i-1]);
        }
        // println!("{:?}", leftsum);
        // println!("{:?}", rightsum);
        leftsum
    }
}

impl Solution2 {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut total = 0;
        let sums = nums.iter().map(|n| {total += n; total}).collect();
        (0..nums.len()).map(|i| Solution::sum_range(&sums, 0, i as i32))
        .zip((0..nums.len()).map(|i| Solution::sum_range(&sums, i as i32, sums.len() as i32-1)))
        .map(|(n1,n2)| (n1-n2).abs()).collect()
    }
    pub fn sum_range(sums: &Vec<i32>, left: i32, right: i32) -> i32 {
        sums[right as usize] - if left == 0 { 0 } else { sums[left as usize-1] }
    }
}

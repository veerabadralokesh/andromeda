impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut c1 = [0; 101];
        let mut c2 = [0; 101];
        for n in &nums1 {c1[*n as usize] += 1};
        for n in &nums2 {c2[*n as usize] += 1};
        let ans1 = nums1.into_iter().filter(|n| c2[*n as usize] > 0).count();
        let ans2 = nums2.into_iter().filter(|n| c1[*n as usize] > 0).count();
        vec![ans1 as i32, ans2 as i32]
    }
}

impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut count1 = 0;
        let mut count2 = 0;
        let mut ans = Vec::new();
        for n1 in &nums1 {
            if nums2.contains(&n1) {
                count1 += 1;
            }
        }
        for n2 in &nums2 {
            if nums1.contains(&n2) {
                count2 += 1;
            }
        }
        ans.push(count1);
        ans.push(count2);
        ans
    }
}

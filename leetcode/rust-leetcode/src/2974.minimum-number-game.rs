impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let mut nums2 = nums.to_vec();
        nums2.sort();
        for i in 0..nums2.len()/2 {
            let x = nums2[2*i];
            let y = nums2[2*i+1];
            nums2[2*i] = y;
            nums2[2*i+1] = x;
        }
        nums2
    }
}
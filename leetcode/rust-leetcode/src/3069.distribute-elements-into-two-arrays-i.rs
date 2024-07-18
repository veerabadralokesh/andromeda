impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr1 = Vec::new();
        arr1.push(nums[0]);
        let mut arr2 = Vec::new();
        arr2.push(nums[1]);
        for i in 2..nums.len() {
            if arr1.last() > arr2.last() {
                arr1.push(nums[i]);
            } else {
                arr2.push(nums[i]);
            }
        }
        arr1.append(&mut arr2);
        arr1
    }
}

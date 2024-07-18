impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];
        for i in 1..nums.len() {
            result[i] = result[i - 1] * nums[i - 1];
        }
        let mut right = 1;
        for i in (0..nums.len()).rev() {
            result[i] *= right;
            right *= nums[i];
        }
        result
    }
}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut products = vec![1; n];
        for i in 1..n {
            products[i] = nums[i-1] * products[i-1];
        }
        let mut right_prod = 1;
        for i in (0..n).rev() {
            products[i] *= right_prod;
            right_prod *= nums[i];
        }
        products
    }
}

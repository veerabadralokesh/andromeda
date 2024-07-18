impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {return 0;}
        let mut prod = 1;
        let mut i = 0;
        let mut ans = 0;
        for (j, n) in nums.iter().enumerate() {
            prod *= n;
            while prod >= k {
                prod /= nums[i];
                i += 1;
            }
            ans += j - i + 1;
        }
        ans as i32
    }
}

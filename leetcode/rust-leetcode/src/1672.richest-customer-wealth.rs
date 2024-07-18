impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut mx: i32 = 0;
        let mut sum = 0;
        for customer in accounts.iter() {
            sum = customer.iter().sum();
            if sum > mx {
                mx = sum;
            }
        }
        mx
    }
}

// LEARN

/*
https://stackoverflow.com/questions/50104999/how-can-you-get-the-sum-of-all-keys-in-an-array-of-structs
https://stackoverflow.com/questions/72540172/how-can-i-filter-a-vector-on-an-index-in-rust
*/

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as u32;
        nums.into_iter().enumerate()
                    .filter(|(i,n)| i.count_ones() == k)
                    .map((|(_, n)| n)).sum::<i32>()//;//.collect::<Vec<i32>>();
    }
}

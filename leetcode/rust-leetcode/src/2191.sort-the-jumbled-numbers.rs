impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mapping = mapping.iter().map(|&m| m as i64).collect::<Vec<_>>();
        let mapnum = |mut n: usize| -> i64 {
            let (mut x, mut prod) = (0, 1);
            loop {
                x += prod * mapping[n % 10];
                n /= 10;
                prod *= 10;
                if n == 0 {
                    break;
                }
            }
            x
        };
        let mut snums = nums.iter().enumerate().map(|(i, &n)| (mapnum(n as usize), i)).collect::<Vec<_>>();
        snums.sort();
        snums.iter().map(|&(_, i)| nums[i]).collect()
    }
}


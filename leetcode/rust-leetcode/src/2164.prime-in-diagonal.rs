impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let mut diags = Vec::with_capacity(4*nums.len());
        for i in 0..nums.len() {
            if nums[i][i] & 1 != 0 || nums[i][i] == 2 {
                diags.push(nums[i][i]);
            }
            if nums[nums.len()-i-1][i] & 1 != 0 || nums[nums.len()-i-1][i] == 2 {
                diags.push(nums[nums.len()-i-1][i]);
            }
        }
        diags.sort_by_key(|&n| -n);
        let is_prime = |n: i32| -> bool {
            if n == 1 {
                return false;
            }
            if n == 2 {
                return true;
            }
            let nf = n as f64;
            let sqn = f64::sqrt(nf) as i32 + 1;
            for i in (3..sqn).step_by(2) {
                if n % i == 0 {
                    return false;
                }
            }
            true
        };
        for &n in diags.iter() {
            if is_prime(n) {
                return n;
            }
        }
        0
    }
}


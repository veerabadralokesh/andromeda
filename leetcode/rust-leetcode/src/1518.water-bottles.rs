impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut ans = 0;
        while num_bottles >= num_exchange {
            let k = num_bottles/num_exchange;
            ans += k * num_exchange;
            num_bottles = num_bottles - k * num_exchange + k;
        }
        ans += num_bottles;
        ans
    }
}


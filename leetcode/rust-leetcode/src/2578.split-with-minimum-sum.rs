impl Solution {
    pub fn split_num(mut num: i32) -> i32 {
        let mut num_vec = vec![];
        while num > 0 {
            num_vec.push(num % 10);
            num /= 10;
        }
        num_vec.sort();
        let (mut num1, mut num2) = (0, 0);
        for i in 0..num_vec.len() {
            if i % 2 == 0 {
                num1 = num1 * 10 + num_vec[i];
            } else {
                num2 = num2 * 10 + num_vec[i];
            }
        }
        num1 + num2
    }
}

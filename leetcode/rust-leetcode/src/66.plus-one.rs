impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        digits.reverse();
        let mut add = 1;
        let mut i = 0;
        while add > 0 {
            if i == digits.len() {
                digits.push(add);
                break;
            }
            digits[i] += add;
            add = 0;
            if digits[i] > 9 {
                digits[i] %= 10;
                add = 1;
            }
            i += 1;
        }
        digits.reverse();
        digits
    }
}

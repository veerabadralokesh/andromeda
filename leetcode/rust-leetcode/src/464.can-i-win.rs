use std::collections::HashMap;
impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if desired_total <= 0 {
            return true;
        }
        let possible_sum = (max_choosable_integer * (max_choosable_integer + 1)) / 2;
        if desired_total > possible_sum {
            return false;
        }

        fn dp(total: i32, mask: i32, memo: &mut HashMap<(i32, i32), bool>, max_choosable_integer: i32) -> bool {
            if total <= 0 {
                return false;
            }
            match memo.get(&(total, mask)) {
                Some(&b) => b,
                None => {
                    let mut b = false;
                    for i in 1..=max_choosable_integer {
                        if (
                            (mask >> i) & 1 == 0 &&
                            !dp(total - i, (mask | (1 << i)), memo, max_choosable_integer)
                        ) {
                            b = true;
                            break;
                        }
                    }

                    memo.insert((total, mask), b);

                    b
                }
            }
        }
        let mut memo = HashMap::new();
        dp(desired_total, 0, &mut memo, max_choosable_integer)
    }
}


// LEARN

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let values = vec![
            (1000,  "M"),
            (900,   "CM"),
            (500,   "D"),
            (400,   "CD"),
            (100,   "C"),
            (90,    "XC"),
            (50,    "L"),
            (40,    "XL"),
            (10,    "X"),
            (9,     "IX"),
            (5,     "V"),
            (4,     "IV"),
            (1,     "I"),
        ];

        let mut num = num;
        let mut result = String::new();

        for &(value, symbol) in &values {
            while num >= value {
                result.push_str(symbol);
                num -= value
            }
        }

        result
    }
}

/*
*/

const ONES : [&str;10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
const TENS : [&str;10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
const CENT : [&str;10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
const MILS : [&str;4]  = ["", "M", "MM", "MMM"];

impl Solution 
{
    pub fn int_to_roman(num: i32) -> String 
    {
        // Given that the number of outcomes is small, a brute force
		// substituion for each power of ten is a viable solution...
		format!("{}{}{}{}", MILS[(num / 1000 % 10) as usize],
                            CENT[(num / 100  % 10) as usize],
                            TENS[(num / 10   % 10) as usize],
                            ONES[(num        % 10) as usize])
    }
}

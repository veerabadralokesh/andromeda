impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        const multiplier: i32 = 26;
        let mut col_num: i32 = 0;
        for ch in column_title.chars() {
            let mut chint: u32 = ch.into();
            chint -= 64;
            // println!("{}, {}", ch, (chint as i32));
            col_num = col_num * multiplier + (chint as i32);
        }
        return col_num;
    }
}

impl Solution2 {
    pub fn title_to_number(column_title: String) -> i32 {
        let alphabet_zero = 'A' as i32 - 1;
        column_title.chars().map(|c| c as i32 - alphabet_zero).fold(0,|acc,x| acc*26 + x)
    }
}

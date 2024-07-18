impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut n = column_number;
        let mut s = Vec::new();;
        while n > 0 {
            s.push((b'A' + (((n-1) % 26) as u8)) as char);
            n = (n-1)/26;
        }
        s.reverse();
        s.iter().collect()
    }
}

/* */

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
      let mut result = String::new();
      while column_number > 0 {
        unsafe {
          let letter = char::from_u32_unchecked(('A' as i32 + (column_number - 1) % 26).try_into().unwrap());
          result.insert(0, letter);
          column_number = (column_number - 1) / 26;
        }
      }
      result
    }
}

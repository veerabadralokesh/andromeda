impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows:usize = num_rows as usize;
        let mut rows:Vec<String> = Vec::with_capacity(num_rows);
        for _ in 0..num_rows {rows.push(String::new());}
        let mut i:usize = 0;
        let l:usize = s.len();
        let mut sc:Vec<char> = s.chars().collect::<Vec<char>>();
        while i < l {
            for j in 0..(num_rows-1).max(1) {
                if i >= l {
                    break;
                }
                rows[j].push(sc[i]);
                i += 1;
            }
            for j in 1..num_rows {
                if i >= l {
                    break;
                }
                rows[num_rows-j].push(sc[i]);
                i += 1;
            }
        }
        rows.join("")
    }
}

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut shift_vec = vec![0; s.len()+1];
        for shift in shifts.iter() {
            let start = shift[0] as usize;
            let end = shift[1] as usize + 1;
            let direction = if shift[2] == 1 {1} else {-1};
            shift_vec[start] += direction;
            shift_vec[end] -= direction;
        }
        let mut sb = s.into_bytes().into_iter().map(|b| b - b'a').collect::<Vec<_>>();
        for i in 0..sb.len() {
            if i > 0 {
                shift_vec[i] += shift_vec[i-1];
            }
            while shift_vec[i] < 0 {
                shift_vec[i] += 26;
            }
            shift_vec[i] %= 26;
            sb[i] = (sb[i] + shift_vec[i] as u8) % 26;
        }
        // println!("{:?}", shift_vec);
        sb.into_iter().map(|b| (b + b'a') as char).collect()
    }
}


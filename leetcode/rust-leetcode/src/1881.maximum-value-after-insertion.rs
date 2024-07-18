impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let mut nb = n.into_bytes().iter().map(|&b| (b-b'0') as i8).collect::<Vec<_>>();
        let x = x as i8;
        let minus = nb[0]==-3;
        let mut i = if minus {1} else {0};
        while i < nb.len() {
            if !minus && nb[i] < x {
                nb.insert(i, x);
                break;
            }
            if minus && nb[i] > x {
                nb.insert(i, x);
                break;
            }
            i += 1;
        }
        if i == nb.len() {
            nb.push(x);
        }
        nb.into_iter().map(|b| ((b + b'0' as i8) as u8) as char).collect()
    }
}


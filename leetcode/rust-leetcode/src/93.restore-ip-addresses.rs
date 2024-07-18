impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let l = s.len();
        if l > 12 || l < 4 {
            return vec![];
        }
        let nums = s.clone().chars().collect::<Vec<char>>();
        let mut addresses:Vec<String> = Vec::new();
        let (lb, ub) = match(l) {
            4 => (1, 1),
            5 => (1, 2),
            11 => (2, 3),
            12 => (3, 3),
            _ => (1, 3),
        };
        let is_valid = |start:usize,end:usize| (end < start + 2 && (end == start || nums[start] > '0'))
        || (nums[start] > '0' && nums[start] < '2') 
        || (nums[start] == '2' 
            && (nums[end-1] < '5' || (nums[end-1]=='5' && nums[end] < '6')));
        let to_string = |start:usize,end:usize| nums[start..end].iter().collect::<String>();

        for l1 in lb..=ub {
            if is_valid(0, l1-1) {
                for l2 in l1+lb..=(l1+ub).min(l) {
                    if is_valid(l1, l2-1) {
                        for l3 in l2+lb..=(l2+ub).min(l) {
                            if is_valid(l2, l3-1) && l3+ub >= l {
                                for l4 in l3+lb..=(l3+ub).min(l) {
                                    if l4 == l && is_valid(l3, l4-1) {
                                        addresses.push(format!("{}.{}.{}.{}", to_string(0, l1), to_string(l1,l2), to_string(l2, l3), to_string(l3, l4)));
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        addresses
    }
}

/* */

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut rv = Vec::new();
        let bytes = s.as_bytes();
        let n = bytes.len();

        if n < 4 {
            return rv;
        }

        let is_num = |from: usize, to: usize| match to - from {
            1 => true,
            2 => bytes[from] != b'0',
            3 => {
                if bytes[from] == b'2' {
                    bytes[from + 1] < b'5' || (bytes[from + 1] == b'5' && bytes[from + 2] <= b'5')
                } else if bytes[from] < b'2' {
                    bytes[from] != b'0'
                } else {
                    false
                }
            }
            _ => false,
        };

        for a in 1..(n - 2) {
            for b in (a + 1)..(n - 1) {
                for c in (b + 1)..n {
                    // println!("{} {} {} {}", a, b, c, n);
                    if is_num(0, a) && is_num(a, b) && is_num(b, c) && is_num(c, n) {
                        let x = [&s[0..a], &s[a..b], &s[b..c], &s[c..n]].join(".");
                        rv.push(x);
                    }
                }
            }
        }

        rv
    }
}

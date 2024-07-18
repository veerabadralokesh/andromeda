use std::collections::HashMap;
impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        // let mut map:BTreeMap<String,i32> = BTreeMap::new();
        // let (mut count) = (0);
        let fc = formula.chars().collect::<Vec<_>>();
        
        fn count(fc: &[char]) -> HashMap<String, i32> {
            // println!("{:?}", fc);
            // let mut stack:Vec<String> = Vec::with_capacity(fc.len());
            let mut i = 0;
            let mut bracket_count = 0;
            let mut map = HashMap::new();
            let mut idx = 0;
            let mut num = 0;
            let mut element = String::new();
            while i < fc.len() {
                match fc[i] {
                    '(' => {
                        if element.len() > 0 {
                            *map.entry(element.clone()).or_insert(0) += num.max(1);
                            element.clear();
                            num = 0;
                        }
                        // stack.push(Element::OpenBracket);
                        bracket_count += 1;
                        idx = i+1;
                        while bracket_count > 0 {
                            i += 1;
                            if fc[i] == '(' {
                                bracket_count += 1;
                            }
                            if fc[i] == ')' {
                                bracket_count -= 1;
                            }
                        }
                        let sub_map = count(&fc[idx..i]);
                        // println!("sub_map {:?}", sub_map);
                        i += 1;
                        while i < fc.len() && fc[i] >= '0' && fc[i] <= '9' {
                            num = num * 10 + (fc[i] as u8 - b'0') as i32;
                            i += 1;
                        }
                        num = num.max(1);
                        for (k, v) in sub_map.into_iter() {
                            *map.entry(k).or_insert(0) += (v * num);
                        }
                        num = 0;
                        i -= 1;
                        // println!("map {:?}", map);
                    },
                    ')' => {
                        
                    },
                    n if n >= '0' && n <= '9' => {
                        num = num * 10 + (n as u8 - b'0') as i32;
                    },
                    c if c >= 'A' && c < 'a' => {
                        if element.len() > 0 {
                            *map.entry(element.clone()).or_insert(0) += num.max(1);
                            element.clear();
                            num = 0;
                        }
                        // stack.push(format!("{c}"));
                        // idx = stack.len() - 1;
                        element.push(c);
                    },
                    c if c >= 'a' => {
                        element.push(c);
                    },
                    _ => unreachable!(),
                }
                i += 1;
            }
            if element.len() > 0 {
                *map.entry(element.clone()).or_insert(0) += num.max(1);
            }
            // println!("{:?}", map);
            map
        }

        let map = count(&fc);

        let mut keys = map.keys().cloned().collect::<Vec<String>>();
        keys.sort();
        
        let mut ans = String::new();
        for k in keys.iter() {
            match map.get(k) {
                Some(1) => ans.push_str(&format!("{k}")),
                Some(c) => ans.push_str(&format!("{k}{c}")),
                None => {}
            }
        }
        ans
    }
}

// #[derive(Debug)]
// enum Element {
//     Atom(String),
//     Count(i32),
//     OpenBracket,
// }

/* */

// LEARN


use std::collections::HashMap;

fn f(it: &mut impl Iterator<Item=char>, counter: &mut HashMap<String, u64>, mul: u64) {
    let mut num = 0;
    let mut p = 1;
    let mut s = vec![];
    while let Some(c) = it.next() {
        match c {
            '0'..='9' => {
                num = num + p*((c as u8 - b'0') as u64);
                p *= 10;
            },
            '(' => break,
            ')' => {
                f(it, counter, num.max(1)*mul);
                num = 0;
                p = 1
            },
            'a'..='z' => {
                s.push(c);
            },
            'A'..='Z' => {
                s.push(c);
                s.reverse();
                let w: String = s.iter().copied().collect();
                s.clear();
                *counter.entry(w).or_default() += num.max(1) * mul;
                num = 0;
                p = 1;
            },
            _ => panic!()
        }
    }
}

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let mut it = formula.chars().rev();
        let mut counter = HashMap::new();
        f(&mut it, &mut counter, 1);
        let mut keys: Vec<String> = counter.keys().cloned().collect();
        keys.sort();
        let mut ans = String::new();
        for k in keys {
            ans.push_str(k.as_str());
            if counter[&k] > 1 {
                ans.push_str(counter[&k].to_string().as_str());
            }
        }
        ans
    }
}


// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        fn recurse(sb: &[u8]) -> NestedInteger {
            let mut stack = vec![];
            let mut num = String::new();
            let mut v:Vec<NestedInteger> = Vec::new();
            let is_list = if sb[0] == b'[' {true} else {false};
            for i in 0..sb.len() {
                if is_list && (i == 0 || i == sb.len()-1) {continue;}
                match sb[i] {
                    b'[' => {
                        stack.push(i);
                    },
                    b']' => {
                        let start = stack.pop().unwrap();
                        let end = i+1;
                        if stack.is_empty() {
                            v.push(recurse(&sb[start..end]));
                        }
                        num.clear();
                    },
                    b',' => {
                        // println!("{:?}", num);
                        if stack.is_empty() && !num.is_empty() {
                            v.push(NestedInteger::Int(num.parse::<i32>().unwrap()));
                        }
                        num.clear();
                    },
                    _ => num.push(sb[i] as char)
                }
            }
            // println!("{:?}", &sb.iter().map(|&b| b as char).collect::<String>());
            // println!("{:?}", v);
            // println!("{:?}", num);
            if is_list {
                if !num.is_empty() {
                    v.push(NestedInteger::Int(num.parse::<i32>().unwrap()));
                }
                NestedInteger::List(v)
            } else {
                NestedInteger::Int(num.parse::<i32>().unwrap())
            }
            // if v.len() == 0 {
            //     NestedInteger::Int(num.parse::<i32>().unwrap())
            // } else {
            //     NestedInteger::List(v)
            // }
        }
        recurse(s.as_bytes())
    }
}


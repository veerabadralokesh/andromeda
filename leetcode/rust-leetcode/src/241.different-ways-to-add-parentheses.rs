use std::collections::HashMap;
impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        fn ways(map: &mut HashMap<(usize, usize), Vec<i32>>, expr: &str, start: usize, end: usize) -> Vec<i32> {
            match map.get(&(start, end)) {
                Some(x) => x.to_vec(),
                None => {
                    let mut ans = vec![];
                    for i in start..end {
                        let c = &expr[i..i+1];
                        if "*+-".contains(c) {
                            for prefix in ways(map, expr, start, i) {
                                for suffix in ways(map, expr, i+1, end) {
                                    match c {
                                        "*" => ans.push(prefix * suffix),
                                        "+" => ans.push(prefix + suffix),
                                        "-" => ans.push(prefix - suffix),
                                        _ => unreachable!(),
                                    }
                                }
                            }
                        }
                    }
                    if ans.is_empty() {
                        ans.push(expr[start..end].parse::<i32>().unwrap());
                    }
                    map.insert((start, end), ans.to_vec());
                    ans
                }
            }
        }
        let mut map = HashMap::new();
        ways(&mut map, &expression, 0, expression.len())
    }
}


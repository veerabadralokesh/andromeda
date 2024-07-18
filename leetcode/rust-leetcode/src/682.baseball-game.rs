impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack = vec![];
        let mut score = 0;
        for o in operations.iter() {
            match o.as_str() {
                "+" => {
                    let l = stack.len();
                    score = stack[l-1] + stack[l-2];
                    stack.push(score);
                },
                "C" => {stack.pop();},
                "D" => {stack.push((*stack.last().unwrap()) << 1);}
                x @ _ => {
                    stack.push((x.parse::<i32>().unwrap()))
                }
            }
        }
        stack.iter().sum()
    }
}


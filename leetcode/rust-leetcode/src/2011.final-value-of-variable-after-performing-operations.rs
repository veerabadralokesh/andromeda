impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut x = 0;
        for op in operations {
            match &op[1..2] {
                "+" => {x += 1;}
                _ => {x -= 1;}
            }
        }
        x
    }
}


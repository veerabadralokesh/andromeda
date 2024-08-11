impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let (mut i, mut j) = (0, 0);
        for command in commands.iter() {
            match command.as_str() {
                "UP" => {
                    i -= 1;
                },
                "DOWN" => {
                    i += 1;
                },
                "LEFT" => {
                    j -= 1;
                },
                _ => {
                    j += 1;
                }
            }
        }
        i * n + j
    }
}


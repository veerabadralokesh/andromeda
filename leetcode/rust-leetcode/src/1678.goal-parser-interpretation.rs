impl Solution {
    pub fn interpret(mut command: String) -> String {
        command.replace("()", "o").replace("(al)", "al")
    }
}

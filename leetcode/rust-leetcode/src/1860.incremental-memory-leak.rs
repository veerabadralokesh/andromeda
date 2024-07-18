impl Solution {
    pub fn mem_leak(mut memory1: i32, mut memory2: i32) -> Vec<i32> {
        let mut program = 1;
        while program <= memory1.max(memory2) {
            if memory1 >= memory2 {
                memory1 -= program;
            } else {
                memory2 -= program;
            }
            program += 1;
        }
        vec![program, memory1, memory2]
    }
}


impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        let ops = [Op::Mul, Op::Div, Op::Add, Op::Sub];
        let mut i = 0;
        let mut clumsy = Vec::with_capacity(2 * n as usize);
        clumsy.push(Op::Int(n));
        for i in 0..n-1 {
            match i % 4 {
                0 => clumsy.push(Op::Mul),
                1 => clumsy.push(Op::Div),
                _ => clumsy.push(Op::Add),
            }
            clumsy.push(Op::Int(
                match i%4 {
                    3 => i-n+1,
                    _ => n-i-1
                }
            ));
        }
        let mut stack = Vec::with_capacity(2*n as usize);
        for i in 0..3 {
            let mut j = 0;
            while j < clumsy.len() {
                if clumsy[j] == ops[i] {
                    if let Some(Op::Int(x)) = stack.pop() {
                        let y = match clumsy[j+1] {
                            Op::Int(y) => y,
                            _ => unreachable!(),
                        };
                        stack.push(
                            match ops[i] {
                                Op::Mul => Op::Int(x * y),
                                Op::Div => Op::Int(x / y),
                                Op::Add => Op::Int(x + y),
                                Op::Sub => Op::Int(x - y),
                                _ => unreachable!(),
                            }
                        );
                        j += 1;
                    }
                } else {
                    stack.push(clumsy[j].clone());
                }
                j += 1;
            }
            // println!("{:?}", clumsy);
            // println!("{:?}", stack);
            clumsy = stack;
            stack = Vec::with_capacity(2*n as usize);
        }
        let ans = match clumsy[0] {
            Op::Int(x) => x,
            _ => unreachable!(),
        };
        ans
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Op {
    Int(i32),
    Mul,
    Div,
    Add,
    Sub,
}



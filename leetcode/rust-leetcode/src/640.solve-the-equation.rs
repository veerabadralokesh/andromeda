impl Solution {
    pub fn solve_equation(equation: String) -> String {

        fn parse_expression(expr: &str) -> (i32, i32) {
            let mut coefficient = 0;
            let mut constant = 0;
            let mut num = 0;
            let mut sign = 1;
            let sc = expr.chars().collect::<Vec<_>>();
            for (i, &c) in sc.iter().enumerate() {
                if c == 'x' {
                    if i > 0 && num == 0 && sc[i-1] == '0' {
                        continue;
                    }
                    coefficient += if num == 0 {sign} else {sign * num};
                    num = 0;
                } else if c == '+' || c == '-' {
                    constant += sign * num;
                    sign = if c == '+' {1} else {-1};
                    num = 0;
                } else {
                    num = num * 10 + ((c as u8) - b'0') as i32;
                }
            }
            constant += sign * num;
            (coefficient, constant)
        }
        
        let (lhs, rhs) = equation.split_once("=").expect("parse error");

        let (left_coefficient, left_constant) = parse_expression(lhs);
        let (right_coefficient, right_constant) = parse_expression(rhs);

        let coeff = left_coefficient - right_coefficient;
        let constant = right_constant - left_constant;

        if coeff == 0 && constant == 0 {
            String::from("Infinite solutions")
        } else if coeff == 0 && constant != 0 {
            String::from("No solution")
        } else {
            format!("x={}", constant/coeff)
        }
    }
}


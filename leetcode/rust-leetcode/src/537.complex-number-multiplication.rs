impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let (n1r, n1i) = num1.split_once('+').expect("hello");
        let a1 = n1r.parse::<i32>().unwrap();
        let b1 = n1i.chars().take(n1i.len()-1).collect::<String>().parse::<i32>().unwrap();
        let (n2r, n2i) = num2.split_once('+').expect("hello");
        let a2 = n2r.parse::<i32>().unwrap();
        let b2 = n2i.chars().take(n2i.len()-1).collect::<String>().parse::<i32>().unwrap();
        let a = a1 * a2 - b1 * b2;
        let b = a1 * b2 + a2 * b1;
        format!("{a}+{b}i")
    }
}


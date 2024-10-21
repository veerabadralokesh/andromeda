impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        Self::parse(&expression, &mut 0)
    }
    fn parse(expr: &String, idx: &mut usize) -> bool {
        let i = *idx;
        if &expr[i..i+1] == "t" {
            *idx += 1;
            return true;
        }
        if &expr[i..i+1] == "f" {
            *idx += 1;
            return false;
        }
        if &expr[i..i+1] == "!" {
            *idx += 2;
            let ans = !Self::parse(expr, idx);
            *idx += 1;
            return ans;
        }
        let isAnd = &expr[i..i+1] == "&";
        let mut ans = isAnd;
        *idx += 2;
        while &expr[(*idx)..(*idx+1)] != ")" {
            let parsed = Self::parse(expr, idx);
            if isAnd {
                ans &= parsed;
            } else {
                ans |= parsed;
            }
            if (&expr[(*idx)..(*idx+1)] == ",") {
                *idx += 1;
            }
        }
        *idx += 1;
        return ans;
    }
}


impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut code = Vec::with_capacity(source.len());
        let mut k = 0;
        let mut multi_line = false;
        let mut code_string = String::new();
        for line in source.iter() {
            k = 0;
            while k < line.len() {
                if k == line.len() - 1 {
                    if !multi_line {
                        code_string.push_str(&line[k..]);
                    }
                    k += 1;
                    break;
                }
                match &line[k..k+2] {
                    "//" => {
                        if !multi_line {
                            break;
                        } else {
                            k += 2;
                        }
                    },
                    "/*" if !multi_line => {
                        multi_line = true;
                        k += 2;
                    },
                    "*/" if multi_line => {
                        k += 2;
                        multi_line = false;
                    }
                    _ => {
                        if !multi_line {
                            code_string.push_str(&line[k..k+1]);
                        }
                        k += 1;
                    }
                }
            }
            if !code_string.is_empty() && !multi_line {
                code.push(code_string.to_string());
                code_string.clear();
            }
        }
        code
    }
}


impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut ans:Vec<i32> = Vec::new();
        for ast in asteroids {
            if ans.len() == 0 || ans[ans.len()-1] * ast > 0 {
                ans.push(ast);
                continue;
            }
            while ans.len() > 0 {
                let k = ans[ans.len() - 1];
                if k * ast > 0 || (ast > 0 && k < 0) {
                    ans.push(ast);
                    break;
                } else if ast < 0 && k > 0 {
                    if ast.abs() == k {
                        ans.pop();
                        break;
                    } else if ast.abs() > k {
                        ans.pop();
                        if ans.len() == 0 {
                            ans.push(ast);
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        ans
    }
}

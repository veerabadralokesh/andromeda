impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        let (mut odd, mut even, mut ans, mut h, mut rc, mut bc) = (1, 2, 0, 0, red, blue);
        while rc >= odd {
            rc -= odd;
            odd += 2;
            h += 1;
            if bc < even {
                break;
            }
            bc -= even;
            even += 2;
            h += 1;
        }
        ans = ans.max(h);
        (odd, even, rc, bc, h) = (1, 2, red, blue, 0);
        while bc >= odd {
            bc -= odd;
            odd += 2;
            h += 1;
            if rc < even {
                break;
            }
            rc -= even;
            even += 2;
            h += 1;
        }
        ans.max(h)
    }
}


impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut change = [0, 0, 0];
        let mut total = 0;
        for &b in bills.iter() {
            if b == 5 {
                total += 5;
                change[0] += 1;
                continue;
            }
            let mut cust_change = b - 5;
            if cust_change > total {return false;}
            while cust_change > 19 && change[2] > 0 {
                change[2] -= 1;
                cust_change -= 20;
            }
            while cust_change > 9 && change[1] > 0 {
                change[1] -= 1;
                cust_change -= 10;
            }
            while cust_change > 4 && change[0] > 0 {
                change[0] -= 1;
                cust_change -= 5;
            }
            if cust_change > 0 {
                return false;
            }
            if b == 10 {change[1]+=1;total += 5;}
            if b == 20 {change[2]+=1;total += 15;}
        }
        true
    }
}


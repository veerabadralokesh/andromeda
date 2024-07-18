impl Solution {
    pub fn dist_money(mut money: i32, children: i32) -> i32 {
        money -= children;
        if money < 0 {
            return -1;
        }
        let mut ans = money / 7;
        money = money % 7;
        if ans == children && money == 0 {
            return ans;
        }
        if ans == children - 1 && money == 3 {
            return ans - 1;
        }
        
        ans.min(children-1)
    }
}


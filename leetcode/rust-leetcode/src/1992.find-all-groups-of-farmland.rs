impl Solution {
    pub fn find_farmland(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(land: &mut Vec<Vec<i32>>, rc: (usize, usize), rcend: &mut (usize, usize)) {
            if rc.0 == land.len() || rc.1 == land[0].len() || land[rc.0][rc.1] != 1 {
                return;
            }
            land[rc.0][rc.1] = 2;
            rcend.0 = rcend.0.max(rc.0);
            rcend.1 = rcend.1.max(rc.1);
            dfs(land, (rc.0+1, rc.1), rcend);
            dfs(land, (rc.0, rc.1+1), rcend);
        }
        let mut ans = Vec::new();
        for i in 0..land.len() {
            for j in 0..land[0].len() {
                if land[i][j] == 1 {
                    let mut rcend = (i, j);
                    dfs(&mut land, (i, j), &mut rcend);
                    ans.push(Vec::from([i as i32, j as i32, rcend.0 as i32, rcend.1 as i32]));
                }
            }
        }
        ans
    }
}

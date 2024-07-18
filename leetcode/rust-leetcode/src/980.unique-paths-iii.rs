impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let (m, n) = (grid.len(), grid[0].len());
        let mut arena = vec![vec![true; n]; m];
        let mut to_visit = 0i32;
        let mut paths = 0i32;
        for i in 0..m {
            for j in 0..n {
                match(&grid[i][j]) {
                    0 => to_visit += 1,
                    1 => {
                        arena[i][j] = false;
                        start = (i, j);
                    },
                    2 => {
                        end = (i, j);
                        to_visit += 1;
                    },
                    _ => {arena[i][j] = false;}
                }
            }
        }
        fn search_arena(arena: &mut Vec<Vec<bool>>, next: (usize,usize), end: &(usize, usize), to_visit: i32, current: i32, paths: &mut i32, mn: &(usize, usize)) {
            if next.0 == end.0 && next.1 == end.1 {
                if to_visit == current {
                    *paths += 1;
                }
                return;
            }
            let nexti = [-1, 0, 0, 1];
            let nextj = [0, -1, 1, 0];
            for idx in 0..4 {
                let i = next.0.saturating_add_signed(nexti[idx]).min(mn.0);
                let j = next.1.saturating_add_signed(nextj[idx]).min(mn.1);
                if arena[i][j] {
                    arena[i][j] = false;
                    search_arena(arena, (i, j), end, to_visit, current+1, paths, mn);
                    arena[i][j] = true;
                }
            }
        }
        search_arena(&mut arena, start, &end, to_visit, 0, &mut paths, &(m-1, n-1));
        paths
    }
}

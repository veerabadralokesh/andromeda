use std::collections::VecDeque;
impl Solution {
    pub fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let k = k as usize;
        let row_order = Self::topological_sort(&row_conditions, k);

        if row_order.is_empty() {
            return vec![]
        }
        
        let col_order = Self::topological_sort(&col_conditions, k);

        if col_order.is_empty() {
            return vec![]
        }

        let mut node_to_row_index = vec![0; k+1];
        for (i, &n) in row_order.iter().enumerate() {
            node_to_row_index[n] = i;
        }
        
        let mut ans = vec![vec![0; k]; k];

        for (j, &n) in col_order.iter().enumerate() {
            ans[node_to_row_index[n]][j] = n as i32;
        }

        ans
    }

    pub fn topological_sort(conditions: &Vec<Vec<i32>>, k: usize) -> Vec<usize> {
        let conditions = conditions.iter().map(|uv| (uv[0] as usize, uv[1] as usize)).collect::<Vec<_>>();
        let mut order = Vec::with_capacity(k);
        let mut graph = vec![vec![]; k+1];
        let mut degree_in = vec![0; k+1];

        for &(u,v) in conditions.iter() {
            graph[u].push(v);
            degree_in[v] += 1;
        }
        let mut q = VecDeque::with_capacity(k);
        for (i, &deg) in degree_in.iter().enumerate().skip(1) {
            if deg == 0 {
                q.push_back(i);
            }
        }

        while let Some(u) = q.pop_front() {
            order.push(u);
            for &v in graph[u].iter() {
                degree_in[v] -= 1;
                if degree_in[v] == 0 {
                    q.push_back(v);
                }
            }
        }

        if order.len() == k {order} else {vec![]}
    }
}


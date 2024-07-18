impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let (sr, sc) = (sr as usize, sc as usize);
        if image[sr][sc] == color {return image;}
        let original_color = image[sr][sc];
        let (m, n) = (image.len(), image[0].len());
        let mut q = std::collections::VecDeque::new();
        q.push_back((sr, sc));
        let get_adjacent_nodes = |x: usize, y: usize| -> Vec<(usize, usize)> {
            let mut nodes = vec![];
            if x > 0 {nodes.push((x-1, y))}
            if x < m-1 {nodes.push((x+1, y))}
            if y > 0 {nodes.push((x, y-1))}
            if y < n-1 {nodes.push((x, y+1))}
            nodes
        };
        while let Some((x, y)) = q.pop_front() {
            image[x][y] = color;
            for node in get_adjacent_nodes(x, y) {
                if image[node.0][node.1] == original_color {
                    q.push_back(node);
                }
            }
        }
        image
    }
}


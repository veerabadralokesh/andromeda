func minimumDiameterAfterMerge(edges1 [][]int, edges2 [][]int) int {
    dia1 := diameter(edges1)
    dia2 := diameter(edges2)
    return max(max(dia1, dia2), (dia1 + 1)/2 + (dia2 + 1)/2 + 1)
}

func max(a int, b int) int {
    if a > b {return a}
    return b
}

func diameter(edges [][]int) int {
    n := len(edges) + 1
    dia := 0
    u := 0
    graph := make([][]int, n)
    for i := range n {
        graph[i] = []int{}
    }
    for _, e := range edges {
        graph[e[0]] = append(graph[e[0]], e[1])
        graph[e[1]] = append(graph[e[1]], e[0])
    }
    degree := make([]int, n)
    visited := make([]bool, n)
    depth := make([]int, n)
    q := []int{}
    for i := range n {
        degree[i] = len(graph[i])
        if degree[i] == 1 {
            q = append(q, i)
        }
    }
    for len(q) > 0 {
        u = q[0]
        q = q[1:]
        visited[u] = true
        for _,v := range graph[u] {
            degree[v]--
            if degree[v] == 1 {
                q = append(q, v)
            }
            if !visited[v] {
                dia = max(dia, depth[u]+depth[v]+1)
                depth[v] = max(depth[v], depth[u] + 1)
            }
        }
    }
    return dia
}


func secondMinimum(n int, edges [][]int, time int, change int) int {
    graph := make([][]int, n+1)
    for _, e := range edges {
        graph[e[0]] = append(graph[e[0]], e[1])
        graph[e[1]] = append(graph[e[1]], e[0])
    }
    times := make([][2]int, n+1)
    for i := range n+1 {
        times[i][0], times[i][1] = math.MaxInt32, math.MaxInt32
    }
    q := make([][2]int, 0, n)
    q = append(q, [2]int{1, 0})
    node := [2]int{0, 0}
    u, t, d, new_t := 0, 0, 0, 0
    for len(q) > 0 {
        node = q[0]
        q = q[1:]
        u, t = node[0], node[1]
        d = t/change
        if d & 1 == 1 {
            t = (d + 1) * change
        }
        new_t = t + time
        for _, v := range graph[u] {
            if new_t < times[v][0] {
                times[v][0] = new_t
                q = append(q, [2]int{v, new_t})
            } else if times[v][0] < new_t && new_t < times[v][1] {
                if v == n {
                    return new_t
                }
                times[v][1] = new_t
                q = append(q, [2]int{v, new_t})
            }
        }
    }
    return 0
}


func maximumImportance(n int, roads [][]int) int64 {
    graph := make([]int, n)
    for _,road := range roads {
        graph[road[0]]++
        graph[road[1]]++
    }
    sort.Sort(sort.Reverse(sort.IntSlice(graph)))
    var ans int64 = 0
    for i:=0; i<n; i++ {
        ans += int64(n-i)*int64(graph[i])
    }
    return ans
}


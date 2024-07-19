func luckyNumbers (matrix [][]int) []int {
    m, n := len(matrix), len(matrix[0])
    rows := make([]int, m)
    cols := make([]int, n)
    for i := range m {
        rows[i] = matrix[i][0]
        for j := range n {
            rows[i] = min(rows[i], matrix[i][j])
            cols[j] = max(cols[j], matrix[i][j])
        }
    }
    set := make(map[int]bool, len(rows))
    for _, r := range rows {
        set[r] = true
    }
    ans := make([]int, 0, m+n)
    for _, c := range cols {
        if _, ok := set[c]; ok {
            ans = append(ans, c)
        }
    }
    return ans
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}


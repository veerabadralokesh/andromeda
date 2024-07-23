func restoreMatrix(rowSum []int, colSum []int) [][]int {
    m, n, cell := len(rowSum), len(colSum), 0
    mat := make([][]int, m)
    for i := 0; i < m ; i++ {
        mat[i] = make([]int, n)
        for j := 0; j < n ; j++ {
            cell = min(rowSum[i], colSum[j])
            mat[i][j] = cell
            rowSum[i] -= cell
            colSum[j] -= cell
        }
    }
    return mat
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}


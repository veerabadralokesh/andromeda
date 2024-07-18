func minFallingPathSum(matrix [][]int) int {
    n := len(matrix)
    for i:=1; i < n; i++ {
        i_minus_1 := i-1
        matrix[i][0] += Min(matrix[i_minus_1][0], matrix[i_minus_1][1])
        for j:=1; j < n-1; j++ {
            matrix[i][j] += Min(matrix[i_minus_1][j-1], Min(matrix[i_minus_1][j], matrix[i_minus_1][j+1]))
        }
        matrix[i][n-1] += Min(matrix[i_minus_1][n-2], matrix[i_minus_1][n-1])
    }
    min_path := 1000000
    for _, path := range matrix[n-1] {
        min_path = Min(min_path, path)
    }
    return min_path
}

func Min(a int, b int) int {
    if a < b {return a}
    return b
}


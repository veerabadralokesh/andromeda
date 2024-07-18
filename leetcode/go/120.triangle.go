func minimumTotal(triangle [][]int) int {
    for row := 1; row < len(triangle); row++ {
        triangle[row][0] += triangle[row-1][0]
        for col := 1; col < row ; col++ {
            triangle[row][col] += Min(triangle[row-1][col-1], triangle[row-1][col])
        }
        triangle[row][row] += triangle[row-1][row-1]
    }
    min_path := 1000000
    lastRow := triangle[len(triangle)-1]
    for _, path := range lastRow {
        min_path = Min(min_path, path)
    }
    return min_path
}

func Min(a int, b int) int {
    if a < b {return a}
    return b
}


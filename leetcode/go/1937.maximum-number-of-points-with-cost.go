func maxPoints(points [][]int) int64 {
    n := len(points[0])
    dp := make([]int, n)
    maxi := 0
    for _, row := range points {
        leftToRight := make([]int, n)
        maxi = 0
        for j := range n {
            maxi = max(maxi-1, dp[j])
            leftToRight[j] = maxi
        }
        maxi = 0
        for j := n-1; j > -1; j-- {
            maxi = max(maxi-1, dp[j])
            dp[j] = max(maxi, leftToRight[j]) + row[j]
        }
    }
    ans := 0
    for _, d := range dp {
        ans = max(ans, d)
    }
    return int64(ans)
}


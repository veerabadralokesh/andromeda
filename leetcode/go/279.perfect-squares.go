func numSquares(n int) int {
    dp := make([]int, n+1)
    for i:= 1; i < n+1; i++ {
        dp[i] = n;
    }
    dp[1] = 1;
    for i := 2; i < n+1; i++ {
        j := 1
        for j * j <= i {
            dp[i] = min(dp[i], dp[i-j*j] + 1)
            j += 1
        }
    }
    return dp[n]
}

func min(a, b int) int {
    if a < b { return a }
    return b
}


func nthUglyNumber(n int) int {
    dp := make([]int, n)
    dp[0] = 1
    head2, head3, head5 := 0, 0, 0
    for i := 1; i < n; i++ {
        dp[i] = min(2 * dp[head2], min(3 * dp[head3], 5 * dp[head5]))
        if dp[i] == 2 * dp[head2] {
            head2++
        }
        if dp[i] == 3 * dp[head3] {
            head3++
        }
        if dp[i] == 5 * dp[head5] {
            head5++
        }
    }
    return dp[n-1]
}

func min(a int, b int) int {
    if a < b {return a}
    return b
}


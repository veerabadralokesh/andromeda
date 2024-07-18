var modulo int64 = 1_000_000_007
func numTilings(n int) int {
    if n == 1 {return 1}
    if n == 2 {return 2}
    if n == 3 {return 5}
    dp := make([]int64, n+1)
    dp[1], dp[2], dp[3] = 1, 2, 5
    for i:=4 ; i<n+1 ; i++ {
        dp[i] = ((dp[i-1] << 1) + dp[i-3]) % modulo
    }
    return int(dp[n])
}


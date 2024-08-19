func minSteps(n int) int {
    dp := make([]int, n+1)
    for i := 2; i < n+1; i++ {
        dp[i] = i
    }
    for i:=2; i < n/2 + 1;i++ {
        ops := dp[i] + 1
        for j:=2*i; j < n+1 ; j+=i {
            ops++
            dp[j] = min(dp[j], ops)
        }
    }
    return dp[n]
}


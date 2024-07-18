func deleteAndEarn(nums []int) int {
    m := make(map[int]int)
    max_num := 0
    for _, n := range nums {
        m[n] += 1
        if n > max_num {
            max_num = n
        }
    }
    dp := make([]int, max_num+1)
    if val, ok := m[1]; ok {
        dp[1] = val
    }
    for i := 2; i < max_num+1; i++ {
        if val, ok := m[i]; ok {
            dp[i] = Max(dp[i-1], (dp[i-2] + i * val))
        } else {
            dp[i] = dp[i-1]
        }
    }
    return dp[max_num]
}

func Max(a int, b int) int {
    if a > b {return a}
    return b
}


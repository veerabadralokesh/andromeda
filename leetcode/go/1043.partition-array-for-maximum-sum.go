func maxSumAfterPartitioning(arr []int, k int) int {
    k++
    n := len(arr)
    dp := make([]int, k)
    end, curr_max := 0, 0
    for start:=n-1; start > -1 ; start-- {
        curr_max = 0
        end = min(start + k - 1, n)
        for i:=start; i<end ; i++ {
            curr_max = max(curr_max, arr[i])
            dp[start % k] = max(
                dp[start % k], 
                dp[(i+1) % k] + (curr_max * (i-start+1)))
        }
    }
    return dp[0]
}

func min(a int, b int) int {
    if a < b {return a}
    return b
}

func max(a int, b int) int {
    if a > b {return a}
    return b
}


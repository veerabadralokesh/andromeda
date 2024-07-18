import (
    "math"
)
func maxDotProduct(nums1 []int, nums2 []int) int {
    m := len(nums1)
    n := len(nums2)
    dp := make([][]int, m+1)
    for i := range m+1 {
        dp[i] = make([]int, n+1)
        for j := range n+1 {
            dp[i][j] = math.MinInt
        }
    }
    for i := range m {
        for j := range n {
            dp[i+1][j+1] = max(
                dp[i+1][j], dp[i][j+1],
                max(0, dp[i][j]) + nums1[i] * nums2[j],
            )
        }
    }
    return dp[m][n]
}


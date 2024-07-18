func maxProfit(prices []int) int {
    maxp, minp, ans := prices[0], prices[0], 0
    for _, p := range prices {
        if p < maxp {
            ans += (maxp - minp)
            minp = p
            maxp = p
        } else if p > maxp {
            maxp = p
        }
        // fmt.Println(p, minp, maxp, ans)
    }
    if maxp > minp {
        ans += (maxp - minp)
    }
    return ans
}

/* */

func maxProfit(prices []int) int {
    ans := 0
    for i:=0; i<len(prices)-1; i++ {
        if prices[i] < prices[i+1] {
            ans += (prices[i+1]-prices[i])
        }
    }
    return ans
}

/* */

func maxProfit(prices []int) int {
    n := len(prices)
    dp := make([][]int, n+1)
    dp[n] = []int{0, 0}
    for i:=n-1; i > -1; i-- {
        dp[i] = []int{0, 0}
        for j := range 2 {
            if j == 1 {
                dp[i][j] = max(dp[i+1][1], dp[i][0]-prices[i])
            } else {
                dp[i][j] = max(prices[i] + dp[i+1][1], dp[i+1][0])
            }
        }
    }
    // for _, row := range dp {
    //     fmt.Println(row)
    // }
    return dp[0][1]
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}


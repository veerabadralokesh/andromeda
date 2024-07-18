func maxProfit(prices []int) int {
    hold1, sell1, hold2, sell2 := math.MinInt32, 0, math.MinInt32, 0

    for _, price := range prices {
        sell2 = max(sell2, hold2 + price)
        hold2 = max(hold2, sell1 - price)
        sell1 = max(sell1, hold1 + price)
        hold1 = max(hold1, -price)
        // fmt.Println(price, sell2, hold2, sell1, hold1)
    }

    return sell2
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}


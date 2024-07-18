func maxProfit(prices []int, fee int) int {
    holding := math.MinInt
    profit := 0
    for _, price := range prices {
        profit = Max(profit, holding + price)
        holding = Max(holding, profit - price - fee)
    }
    return profit
}

func Max(a int, b int) int {
    if a > b {return a}
    return b
}


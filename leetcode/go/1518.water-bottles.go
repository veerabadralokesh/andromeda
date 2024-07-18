func numWaterBottles(numBottles int, numExchange int) int {
    ans := 0
    for numBottles >= numExchange {
        x := numBottles/numExchange
        ans += x * numExchange
        numBottles -= (x * numExchange - x)
    }
    ans += numBottles
    return ans
}


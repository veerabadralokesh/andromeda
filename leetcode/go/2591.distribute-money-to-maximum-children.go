func distMoney(money int, children int) int {
    money -= children
    if money < 0 {
        return -1
    }
    ans := money / 7
    money %= 7
    if ans == children && money == 0 {
        return ans
    }
    if ans == children-1 && money == 3 {
        return ans - 1
    }
    return min(children-1, ans) 
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}


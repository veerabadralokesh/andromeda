func missingRolls(rolls []int, mean int, n int) []int {
    rem := (n + len(rolls)) * mean
    for _, roll := range rolls {
        rem -= roll
    }
    if rem > n * 6 || rem < n {
        return []int{}
    }
    ans := make([]int, n)
    for i:=0; i < n; i++ {
        ans[i] = rem/n
    }
    rem = rem % n
    for i:=0; i < rem; i++ {
        ans[i]++
    }
    return ans
}


func minSubarray(nums []int, p int) int {
    sum := 0
    for _, n := range nums {
        sum += n
    }
    rem := sum % p
    if rem == 0 {
        return 0
    }
    ans := len(nums)
    idxs := make(map[int]int)
    idxs[0] = -1

    prefix, target := 0, 0
    for i, n := range nums {
        prefix = (prefix + n) % p
        target = (prefix - rem + p) % p
        if idx, ok := idxs[target]; ok {
            ans = min(ans, i - idx)
        }
        idxs[prefix] = i
    }
    if ans == len(nums) {
        ans = -1
    }
    return ans
}


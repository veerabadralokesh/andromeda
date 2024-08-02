func minSwaps(nums []int) int {
    l := len(nums)
    ones, zeros := 0, 0
    for _, n := range nums {
        if n == 1 {
            ones++
        }
    }
    for i:=0; i<ones;i++ {
        if nums[i] == 0 {
            zeros++
        }
    }
    ans := min(l-ones, zeros)
    for i:=ones; i<l+ones; i++ {
        if nums[i - ones] == 0 {
            zeros--
        }
        if nums[i % l] == 0 {
            zeros++
        }
        ans = min(ans, zeros)
    }
    return ans
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}


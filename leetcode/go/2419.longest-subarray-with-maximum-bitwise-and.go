func longestSubarray(nums []int) int {
    maxn := nums[0]
    for _, n := range nums {
        maxn = max(maxn, n)
    }
    count, ans := 0, 1
    for _, n := range nums {
        if n == maxn {
            count++
            ans = max(ans, count)
        } else {
            count = 0
        }
    }
    return ans
}


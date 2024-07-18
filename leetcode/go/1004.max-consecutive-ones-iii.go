func longestOnes(nums []int, k int) int {
    left, ans, count := 0, 0, 0
    for right, n := range nums {
        if n == 0 {
            count++
        }
        for count > k {
            if nums[left] == 0 {
                count--
            }
            left++
        }
        ans = max(ans, right-left+1)
    }
    return ans
}

func max(a, b int) int {
    if a < b { return b }
    return a
}


func maxRotateFunction(nums []int) int {
    sum := 0
    dp := 0
    for i,n := range nums {
        sum += n
        dp += i * n
    }
    l := len(nums)
    ans := dp
    for i:=1; i < l; i++ {
        dp = dp + sum - l * nums[l-i]
        ans = max(ans, dp)
    }
    return ans
}


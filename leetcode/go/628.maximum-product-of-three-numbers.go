func maximumProduct(nums []int) int {
    n := len(nums)
    slices.Sort(nums)
    x, y := nums[0]*nums[1]*nums[n-1], nums[n-1] * nums[n-2] * nums[n-3]
    if x > y {
        return x
    }
    return y
}


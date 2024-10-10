func maxWidthRamp(nums []int) int {
    stack := make([]int, 0, len(nums))
    idx := 0
    for i, n := range nums {
        if len(stack) == 0 || nums[idx] >= n {
            idx = i
            stack = append(stack, i)
        }
    }
    ans := 0
    l := len(stack)-1
    for i:=len(nums)-1; i > -1; i-- {
        for l > -1 && nums[stack[l]] <= nums[i] {
            ans = max(ans, i - stack[l])
            l--
        }
    }
    return ans
}


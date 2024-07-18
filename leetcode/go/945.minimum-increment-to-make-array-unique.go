func minIncrementForUnique(nums []int) int {
    slices.Sort(nums)
    increment := 0
    for i := 1 ; i < len(nums) ; i++ {
        if nums[i] <= nums[i-1] {
            increment += (nums[i-1] - nums[i]) + 1
            nums[i] = nums[i-1] + 1
        }
    }
    return increment
}


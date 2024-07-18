func sortColors(nums []int)  {
    if len(nums) < 2 {
        return;
    }
    left, right, i := 0, len(nums)-1, 0
    for i <= right && right > 0 {
        if nums[i] == 0 && i != left {
            nums[i], nums[left] = nums[left], nums[i]
            left++
        } else if nums[i] == 2 {
            nums[i], nums[right] = nums[right], nums[i]
            right--
        } else {
            i++
        }
    }
}


func isMonotonic(nums []int) bool {
    increasing := true
    is_set := false
    for i:=1; i < len(nums); i++ {
        if nums[i] == nums[i-1] {
            continue
        }
        if !is_set {
            is_set = true
            if nums[i] < nums[i-1] {
                increasing = false
            }
        } else {
            if nums[i] > nums[i-1] {
                if !increasing {
                    return false
                }
            } else {
                if increasing {
                    return false
                }
            }
        }
    }
    return true
}


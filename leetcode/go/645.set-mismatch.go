func findErrorNums(nums []int) []int {
    duplicate := 0
    for _, n := range nums {
        x := Abs(n)-1
        if nums[x] < 0 {
            duplicate = x + 1
        } else {
            nums[x] *= -1
        }
    }
    for i, n := range nums {
        if n > 0 {
            return []int{duplicate, i+1}
        }
    }
    return []int{0, 0}
}

func Abs(x int) int {
    if x < 0 {
        return -x
    }
    return x
}


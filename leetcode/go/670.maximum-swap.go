func maximumSwap(num int) int {
    nums := []rune(strconv.Itoa(num))
    lastIdx := [10]int{-1, -1, -1, -1, -1, -1, -1, -1, -1, -1}
    for i, x := range nums {
        lastIdx[x - '0'] = i
    }
    for i:=0; i < len(nums); i++ {
        for j:=9; j > int(nums[i]-'0'); j-- {
            if lastIdx[j] > i {
                nums[lastIdx[j]], nums[i] = nums[i], nums[lastIdx[j]]
                // fmt.Println()
                ans, _ := strconv.Atoi(string(nums))
                return ans
            }
        }
    }
    return num
}


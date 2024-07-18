func sequentialDigits(low int, high int) []int {
    nums := []int {1,2,3,4,5,6,7,8,9}
    idx_to_num := func(start int, end int) int {
        x := nums[start]
        for j:=start+1; j < end; j++ {
            x = x * 10 + nums[j]
        }
        return x
    }
    n := 10
    ans := []int{}
    for length := len(strconv.Itoa(low)); length < len(strconv.Itoa(high))+1 ; length++ {
        for start := 0 ; start < n-length; start++ {
            num := idx_to_num(start, start + length)
            if num < low {
                continue
            }
            if num > high {
                break
            }
            ans = append(ans, num)
        }
    }
    return ans
}


func arrayNesting(nums []int) int {
    n := len(nums)
    visited := make([]bool, n)
    ans := 0
    for i := range n {
        if !visited[i] {
            setLength := 0
            j := i
            for !visited[j] {
                visited[j] = true
                j = nums[j]
                setLength++
                ans = max(ans, setLength)
            }
        }
    }
    return ans
}


func maximumDifference(nums []int) int {
    if len(nums) < 2 {
        return -1
    }
    s := make(stack, 0)
    ans := -1
    l := len(nums)
    s = s.Push(nums[l-1])
    for i:=l-2; i > -1 ; i-- {
        if nums[i] >= s.Peek() {
            s = s.Push(nums[i])
        } else {
            ans = Max(ans, s.Peek() - nums[i])
        }
    }
    return ans
}

func Max(a int, b int) int {
    if a > b {return a}
    return b
}

type stack []int

func (s stack) Peek() int {
    return s[len(s)-1]
}

func (s stack) Push(v int) stack {
    return append(s, v)
}

func (s stack) Pop() (stack, int) {
    l := len(s)
    return  s[:l-1], s[l-1]
}


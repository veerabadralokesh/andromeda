func findClosestNumber(nums []int) int {
    ans := 10000000
    dist := ans
    for _, n := range nums {
        y := Abs(n)
        if y < dist {
            ans = n
            dist = y
        } else if y == dist {
            ans = Max(ans, n)
        }
    }
    return ans
}

func Abs(x int) int {
    if x < 0 {return -x}
    return x
}

func Max(x int, y int) int {
    if x > y {return x}
    return y
}


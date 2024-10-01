func canArrange(arr []int, k int) bool {
    rems := make([]int, k)

    for _, n := range arr {
        rems[((n % k) + k) % k]++
    }

    if rems[0] & 1 != 0 {
        return false
    }

    for i:=1; i < 1 + k/2; i++ {
        if rems[i] != rems[k-i] {
            return false
        }
    }

    return true
}


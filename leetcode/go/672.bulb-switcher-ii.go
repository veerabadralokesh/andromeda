func flipLights(n int, presses int) int {
    if n > 3 {
        n = 3
    }
    switch presses {
        case 0:
            return 1
        case 1:
            return []int{2,3,4}[n-1]
        case 2:
            return []int{2,4,7}[n-1]
        default:
            return []int{2,4,8}[n-1]
    }
}


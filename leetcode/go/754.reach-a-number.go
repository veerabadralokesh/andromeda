func reachNumber(target int) int {
    if target < 0 {
        target = -target
    }
    i := 0
    x := 0
    for x != target {
        if x < target {
            i++
            x += i
        } else {
            diff := x - target
            if diff & 1 == 0 {
                break;
            } else {
                x = target - 1
            }
        }
    }
    return i
}


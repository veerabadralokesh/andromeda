func threeConsecutiveOdds(arr []int) bool {
    arr[0] = arr[0] & 1
    for i:=1; i < len(arr); i++ {
        if arr[i] & 1 == 0 {
            arr[i] = 0
        } else {
            arr[i] = 1 + arr[i-1];
            if arr[i] == 3 {
                return true
            }
        }
    }
    return false
}


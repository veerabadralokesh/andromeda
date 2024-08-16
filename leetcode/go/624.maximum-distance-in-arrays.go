func maxDistance(arrays [][]int) int {
    ans, mini, maxi, last := 0, 1000000, -1000000, 0
    for _, arr := range arrays {
        last = len(arr)-1
        ans = max(ans, maxi - arr[0], arr[last] - mini)
        maxi = max(arr[last], maxi)
        mini = min(arr[0], mini)
    }
    return ans
}


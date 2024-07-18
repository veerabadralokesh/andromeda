func earliestFullBloom(plantTime []int, growTime []int) int {
    plantGrow := make([][]int, len(plantTime))
    for i, p := range plantTime {
        plantGrow[i] = make([]int, 2)
        plantGrow[i][0] = p
        plantGrow[i][1] = growTime[i]
    }
    sort.Slice(plantGrow, func(i, j int) bool {
        return plantGrow[i][1] > plantGrow[j][1]
    })
    ans, time := 0, 0
    for _, pg := range plantGrow {
        time += pg[0]
        ans = max(ans, time + pg[1])
    }
    return ans
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}


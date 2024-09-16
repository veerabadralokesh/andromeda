import (
    "fmt"
    "sort"
    "strconv"
    "strings"
)

func findMinDifference(timePoints []string) int {
    minutes := make([]int, len(timePoints))
    for i, timePoint := range timePoints {
        hour, _ := strconv.Atoi(timePoint[:2])
        minute, _ := strconv.Atoi(timePoint[3:])
        minutes[i] = hour * 60 + minute
    }

    sort.Ints(minutes)

    ans := 1440
    for i := 1; i < len(minutes); i++ {
        ans = min(ans, minutes[i]-minutes[i-1])
    }

    ans = min(ans, 1440-minutes[len(minutes)-1]+minutes[0])

    return ans
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}


func hIndex(citations []int) int {
    sort.Sort(sort.Reverse(sort.IntSlice(citations)))
    h := 0
    for _, c := range citations {
        if c <= h {
            break;
        }
        h++
    }
    return h
}

/* */

func hIndex(citations []int) int {
  n := len(citations)
  dp := make([]int, n+1)
  for i := 0; i < n; i++ {
    cur := citations[i]
    if cur > n {
      cur = n
    }
    for cur > 0 {
      dp[cur]++
      cur--
    }
  }
  for i := n; i >= 0; i-- {
    if dp[i] >= i {
      return i
    }
  }
  return 0
}


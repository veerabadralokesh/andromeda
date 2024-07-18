type IntHeap []int
func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *IntHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *IntHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func findMaximizedCapital(k int, w int, profits []int, capital []int) int {
    n := len(profits)
    cap_profit := make([][]int, n)
    for i, c := range capital {
        cap_profit[i] = []int{c, profits[i]}
    }
    sort.Slice(cap_profit, func(i, j int) bool {
        return cap_profit[i][0] < cap_profit[j][0]
    })
    h := make(IntHeap, n)
    heap.Init(&h)
    i := 0
    for j := 0; j < k; j++ {
        for i < n && cap_profit[i][0] <= w {
            heap.Push(&h, cap_profit[i][1])
            i++
        }
        if h.Len() == 0 {
            break
        }
        w += heap.Pop(&h).(int)
    }
    return w
}


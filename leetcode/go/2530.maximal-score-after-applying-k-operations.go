type IntHeap []int
func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Peek() int           { return h[0] }
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


func maxKelements(nums []int, k int) int64 {
    maxHeap := make(IntHeap, len(nums))
    heap.Init(&maxHeap)
    for _, n := range nums {
        heap.Push(&maxHeap, n)
    }

    var ans int64 = 0
    maxNum := 0
    for i:=0 ; i<k; i++ {
        maxNum = heap.Pop(&maxHeap).(int)
        ans += int64(maxNum)
        heap.Push(&maxHeap, (maxNum+2)/3)
    }
    return ans
}

